use crate::*;

pub async fn query_uisat_address(address: &str) -> InscriptionByAddress{
    let resp = reqwest::get(format!("https://unisat.io/api/v2/address/inscriptions?address={}", address))
        .await.unwrap()
        .json::<InscriptionByAddress>()
        .await.unwrap();
    resp
}   
