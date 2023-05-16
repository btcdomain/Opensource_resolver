use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "kebab-case")]
pub struct InscribeInfo {
    pub id: u64,
    pub inscribe_num: u64,
    pub inscribe_id: String,
    pub sat: u64,
    pub domain_name: String,
    pub address: String,
    pub create_time: u64,
    pub update_time: u64,
    pub expire_date: i64,
    pub register_date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryNumber {
    pub number: u64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DomainInfo {
    pub id: u64,
    pub domain_name: String,
    pub inscribe_num: u64,
    pub inscribe_id: String,
    pub owner_address: String,
    pub content: String,
    pub create_time: u64,
    pub update_time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscribeContent {
    pub content: Vec<u8>,
    pub inscribe_num: u64,
    pub inscribe_id: String,
    pub sat: u64,
    pub address: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct InscribeData {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub first_owner: String,
    #[serde(rename = "createDate")]
    pub create_date: u64,
    #[serde(rename = "registerDate")]
    pub register_date: u64,
    #[serde(rename = "expireDate")]
    pub expire_date: u64,
    pub img_url: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscribeSignData {
    pub name: String,
    pub first_owner: String,
    #[serde(rename = "createDate")]
    pub create_date: u64,
    #[serde(rename = "registerDate")]
    pub register_date: u64,
    #[serde(rename = "expireDate")]
    pub expire_date: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InscribeResponse<T, M> {
    pub code: i32,
    pub data: T,
    pub message: M,
}