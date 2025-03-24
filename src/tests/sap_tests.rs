#[cfg(test)]
mod sap_tests{
    use ark_bn254::{Fr, G1Affine};
    use mpc_curvy::off_chain::{recipient::{scan, RecipientRequest, RecipientResponse}, sender::{send, SenderRequest, SenderResponse}, utils::{generate_bn254_key_pair, generate_secp256k1_key_pair, serialize_affine_point, serialize_field_element, serialize_secp_pk, serialize_secret_key}};
    use secp256k1::{PublicKey, SecretKey};

    
   
    #[test]
    fn test_sap_e2e(){
        fn gen_key() -> (String, Fr, G1Affine, SecretKey, PublicKey) {
            let (v, viewing_pk) = generate_bn254_key_pair(); 
            let (k, spending_pk) = generate_secp256k1_key_pair(); 
            
            let request = SenderRequest{
                viewing_pub_key: serialize_affine_point(&viewing_pk).unwrap(), 
                spending_pub_key:  serialize_secp_pk(&spending_pk), 
                view_tag_version: 0 
            }; 
            let request = serde_json::to_string(&request).unwrap();

            (request, v, viewing_pk, k, spending_pk)
        }
        fn send_money(request: &String, ephemeral_key_reg: &mut Vec<String>, viewtags: &mut Vec<String>, stealth_addresses: &mut Vec<String>){
            let response = send(&request).unwrap();
            let response: SenderResponse = serde_json::from_str(&response).unwrap(); 

            ephemeral_key_reg.push(response.ephemeral_pub_key); 
            viewtags.push(response.view_tag); 
            stealth_addresses.push(response.stealth_address); 
        }

        let mut ephemeral_key_reg: Vec<String> = Vec::new(); 
        let mut viewtags: Vec<String> = Vec::new(); 
        let mut stealth_addresses: Vec<String> = Vec::new(); 


        let (request, v, _, k, _) = gen_key(); 

        send_money(&request, &mut ephemeral_key_reg, &mut viewtags, &mut stealth_addresses);
        let stealth_address = stealth_addresses[0].clone(); 

        for _ in 0..50{
            let (request, _, _, _, _) = gen_key(); 

            send_money(&request, &mut ephemeral_key_reg, &mut viewtags, &mut stealth_addresses);
        }
         
        let request = RecipientRequest{
            ephemeral_pub_key_reg: ephemeral_key_reg, 
            viewtags, 
            view_tag_version: 0, 
            viewing_sk: serialize_field_element(&v), 
            spending_sk: serialize_secret_key(&k)
        }; 
        let request = serde_json::to_string(&request).unwrap();

        let response = scan(&request).unwrap(); 
        let response: RecipientResponse = serde_json::from_str(&response).unwrap();

        assert!(response.stealth_addresses.contains(&stealth_address))
    }

}