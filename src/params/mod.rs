use std::env;


pub fn get_proof_file() -> String{
    get_env_str( "proof_file", "http://3.236.219.234/file")
}

pub fn get_env_str(name: &str, def: &str) -> String {
    if let Ok(str) = env::var(name.clone()) {
        str
    }else {
        def.to_string()
    }
}