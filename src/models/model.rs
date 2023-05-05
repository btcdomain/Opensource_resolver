use diesel::{Insertable, AsChangeset, Queryable};
use rocket::{serde::{Serialize, Deserialize}};
use crate::schemas::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscriptionByAddress {
    pub status: String,
    pub message: String,
    pub result: Vec<InscriptionByAddressData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscriptionByAddressData {
    pub id: String,
    pub num: i64,
    pub number: u64,
    pub detail: InscriptionByAddressDetail,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct InscriptionByAddressDetail {
    pub id: String,
    pub address: String,
    pub output_value: u64,
    pub preview: String,
    pub content: String,
    pub content_length: u64,
    pub content_type: String,
    pub timestamp: String,
    pub genesis_transaction: String,
    pub location: String,
    pub output: String,
    pub offset: u64,
    pub content_body: String,
}

pub fn default_string() -> String {
    String::new()
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BtcDomainLink {
    #[serde(rename = "type")]
    pub _type: String,
    pub domain: String,
    pub obj_ins_id: String,
    pub public_key: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct BtcDomainLinkSign {
    #[serde(rename = "type")]
    pub _type: String,
    pub domain: String,
    pub obj_ins_id: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct InscribeIdContent {
    pub content: Vec<u8>,
    pub content_type: String,
    pub inscribe_num: i64,
    pub address: Vec<String>,
}