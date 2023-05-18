use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct ResolveResp {
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeResponse<T, M> {
    pub code: i32,
    pub data: T,
    pub message: M,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeInfoResp {
    pub inscribe_num: i64,
    pub inscribe_id: String,
    pub domain_name: String,
    pub address: String,
    pub create_date: i64,
    pub expire_date: i64,
    pub register_date: i64,
    pub img_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeContent {
    pub content: Vec<u8>,
    pub inscribe_num: u64,
    pub inscribe_id: String,
    pub timestamp: u32,
    pub address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeContentSimple {
    pub content: Vec<u8>,
    pub inscribe_num: i64,
    pub inscribe_id: String,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeIdContent {
    pub content: Vec<u8>,
    pub content_type: String,
    pub inscribe_num: i64,
    pub output_address: String,
    pub input_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeAddr {
    pub output_address: String,
    pub input_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct CurMaxInsNum {
    pub inscribe_num: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct VerifyData {
    pub data: Vec<u8>,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeData {
    #[serde(rename = "type")]
    pub type_: String,
    pub name: String,
    pub first_owner: String,
    #[serde(rename = "createDate")]
    pub create_date: i64,
    #[serde(rename = "registerDate")]
    pub register_date: i64,
    #[serde(rename = "expireDate")]
    pub expire_date: i64,
    pub img_url: String,
    pub sig: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscribeSignData {
    pub name: String,
    pub first_owner: String,
    #[serde(rename = "createDate")]
    pub create_date: i64,
    #[serde(rename = "registerDate")]
    pub register_date: i64,
    #[serde(rename = "expireDate")]
    pub expire_date: i64,
}