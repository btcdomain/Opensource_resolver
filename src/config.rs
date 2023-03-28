use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub const START_INSCRIPTION_NUMBER: u64 = 325000;
pub const SUCCESS: i32 = 0i32;
pub const ERROR_1: i32 = 1i32;
pub const ERROR_2: i32 = 2i32;
pub const DEFAULT_IMG_URL: &str = "https://btcdomains.io/images/domain";
pub const PROGRAM_HASH: &str = "0x69ba56d1a366f02710e30446b55360392456bc7e5502bbb6227130c0c9e1080";

pub fn get_database_url() -> String{
    get_env_str( "database", "None")
}

pub fn get_env_str(name: &str, def: &str) -> String {
    if let Ok(str) = env::var(name.clone()) {
        str
    }else {
        def.to_string()
    }
}

pub fn get_now_time() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
}