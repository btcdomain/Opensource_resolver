use std::env;

use std::time::Duration;
use diesel::MysqlConnection;
use diesel::{RunQueryDsl, QueryDsl};
use diesel::result::Error;
use lazy_static::lazy_static;
use dotenv::dotenv;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::ExpressionMethods;

use crate::{DomainInscriptionInfo, get_now_time};
use crate::domain_inscription_info;
use crate::domain_inscription_info::{domain_name, inscribe_num};
use crate::domain_inscription_info::dsl::*;

lazy_static! {
    pub static ref POOL: Pool<ConnectionManager<MysqlConnection>> = get_pool_config();
}

pub fn get_pool_config() -> Pool<ConnectionManager<MysqlConnection>> {
    // let database_url = "mysql://root:123456@127.0.0.1:3306/domain_inscription_data";
    dotenv().ok();
    let database_url = env::var("database_url").expect("database url must be set");
    let manager: ConnectionManager<MysqlConnection> = diesel::r2d2::ConnectionManager::new(database_url);
    let pool = diesel::r2d2::Pool::builder()
        .max_size(10)
        .connection_timeout(Duration::from_secs(5u64))
        .build(manager).unwrap();
    pool
}


pub fn insert_inscribe_info(info: DomainInscriptionInfo) -> Result<usize, Error> {
    let mut conn = POOL.get().unwrap();
    let result = diesel::insert_into(domain_inscription_info::table)
        .values(&info)
        .execute(&mut conn)
        .expect("Create data failed");

    Ok(result)
}

pub fn query_by_domain(domain: &str) -> Result<DomainInscriptionInfo, Error> {
    let mut conn = POOL.get().unwrap();
    let result = domain_inscription_info.filter(domain_name.eq(domain)).get_result(&mut conn).unwrap();

    Ok(result)
}

pub fn query_by_number(num: i64) -> Result<DomainInscriptionInfo, Error> {
    let mut conn = POOL.get().unwrap();
    let result = domain_inscription_info.filter(inscribe_num.eq(num)).get_result(&mut conn).unwrap();

    Ok(result)
}

pub fn query_by_address(addr: &str) -> Result<Vec<DomainInscriptionInfo>, Error> {
    let mut conn = POOL.get().unwrap();
    let result = domain_inscription_info.filter(address.eq(addr)).get_results(&mut conn).unwrap();

    Ok(result)
}

pub fn query_lastest_number() -> Result<i64, Error> {
    let mut conn = POOL.get().unwrap();
    let result = domain_inscription_info::table.order(domain_inscription_info::id.desc()).limit(1).load::<DomainInscriptionInfo>(&mut conn);
    if result.is_ok() {
        Ok(result.unwrap()[0].id)
    }else {
        Ok(0i64)
    }
    
}

pub fn query_all() -> Result<Vec<DomainInscriptionInfo>, Error> { 
    let mut conn = POOL.get().unwrap();
    let result = domain_inscription_info::table.order(domain_inscription_info::id.desc()).load::<DomainInscriptionInfo>(&mut conn);
    result
}

pub fn update_info_time(data_id: i64) -> Result<usize, Error> {
    let mut conn = POOL.get().unwrap();
    let result = diesel::update(domain_inscription_info::table.find(data_id))
        .set(domain_inscription_info::update_time.eq(get_now_time()))
        .execute(&mut conn);

    result
}

pub fn update_info_address(data_id: i64, new_address: &str) -> Result<usize, Error> {
    let mut conn = POOL.get().unwrap();
    let result = diesel::update(domain_inscription_info::table.find(data_id))
        .set(
            (
                domain_inscription_info::update_time.eq(get_now_time()),
                domain_inscription_info::address.eq(new_address)
            )
        )
        .execute(&mut conn);

    result
}

pub fn delete_info(data_id: i64) -> Result<usize, Error> {
    let mut conn = POOL.get().unwrap();
    diesel::delete(domain_inscription_info::table.find(data_id)).execute(&mut conn)
}