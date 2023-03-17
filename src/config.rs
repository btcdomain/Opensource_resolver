use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub const START_INSCRIPTION_NUMBER: u64 = 325000;

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