use crate::*;

pub async fn query_uisat_address(address: &str) -> InscriptionByAddress{
    let resp = reqwest::get(format!("https://unisat.io/api/v2/address/inscriptions?address={}", address))
        .await.unwrap()
        .json::<InscriptionByAddress>()
        .await.unwrap();
    resp
}   

pub async fn query_content_by_id(inscription_id: &str) -> Option<String> {
    // let url = format!("https://ord-mirror.magiceden.dev/content/{}", inscription_id);
    let url = format!("https://ordinals.com/content/{}", inscription_id);
    query_by_url(&url).await
}

pub async fn query_by_url(url: &str) -> Option<String>{
    let resp = reqwest::get(url)
        .await.unwrap()
        .text()
        .await.ok();
    resp
}