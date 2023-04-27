use std::env;

use std::time::Duration;
use diesel::MysqlConnection;
use lazy_static::lazy_static;
use dotenv::dotenv;
use diesel::r2d2::{Pool, ConnectionManager};
use log::info;


lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<MysqlConnection>> = get_pool_config();
}

pub fn get_pool_config() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();
    let database_url = env::var("database_url").expect("database url must be set");
    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::new(database_url);
    let pool = Pool::builder()
        .max_size(10)
        .connection_timeout(Duration::from_secs(5u64))
        .build(manager).unwrap();
    pool
}


mod domain_repo;
pub use domain_repo::*;

mod black_repo;
pub use black_repo::*;