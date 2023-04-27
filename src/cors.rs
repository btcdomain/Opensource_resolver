use std::str::FromStr;
use rocket_cors::{Cors, AllowedOrigins, AllowedHeaders};

pub fn get_cors() -> Cors {
	// 允许访问的域，这里允许全部，如果要指定其他可以
    let allowed_origins = AllowedOrigins::All;
    rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec!["Get", "Post", "Options"].iter().map(|s| FromStr::from_str(s).unwrap()).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }.to_cors().expect("cors config error")
}