use crate::*;

pub async fn query_uisat_address(address: &str) -> InscriptionByAddress{
    let resp = reqwest::get(format!("https://unisat.io/api/v2/address/inscriptions?address={}", address))
        .await.unwrap()
        .json::<InscriptionByAddress>()
        .await.unwrap();
    resp
}

pub async fn query_content(ins_id: &str) -> Vec<u8>{
    let result = reqwest::get(format!("https://ordinals.com/content/{}", ins_id))
        .await.unwrap()
        .bytes()
        .await.unwrap();
    result.to_vec()
} 
