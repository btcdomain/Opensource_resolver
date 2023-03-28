use btcdomain_resolver::*;

fn main() {
    let domain = String::from("free.btc");
    let sign_info = InscribeSignData{
        name: domain.clone(),
        first_owner: String::from("bc1px3hey79zhn87vkj7y4hgmkzu3glzqnzhu6fawm6ape5p89l4n77qcy8t0m"),
        create_date: 1678280074069,
        register_date: 1678280074069,
        expire_date: 1709816074069
    };
    let sign_data = serde_json::to_vec(&sign_info).unwrap();
    let verify_data = VerifyData {
        data: sign_data,
        signature: String::from("3045022100dc6d739ae0667f5bc0cfe7d9a52f94ab3bddd137d84aa6e87e6f1ba6007696ea02204a7c642772206c0907a71220732a9bc03d8fda5c68b8a050d01e0f2d6261f8ff")
    };

    let proof = generate_proof(&verify_data, &domain);
    if proof.is_some() {
        println!("proof: {:?}", proof.unwrap().len());
    }else {
        println!("proof: {:?}", proof);
    }
    
}