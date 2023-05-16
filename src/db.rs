use std::time::Duration;
use std::thread;
use mysql::{*, prelude::{Queryable}};
use log::info;
use lazy_static::lazy_static;
use crate::config::*;
use crate::entity::*;

lazy_static!{
    pub static ref POOL: Pool = get_pool();
}

fn get_pool() -> Pool {
    let url = get_database_url();
    let pool = Pool::new(url.as_str());
    if pool.is_ok() {
        pool.unwrap()
    }else {
        thread::sleep(Duration::from_secs(5));
        get_pool()
    }
}

pub fn insert_inscribe_info(info: InscribeInfo) -> Result<()> {
    let mut conn = POOL.get_conn().unwrap();
    let result = conn.exec_drop("INSERT INTO domain_inscription_info(inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date) values (:inscribe_num, :inscribe_id, :sat, :domain_name, :address, :create_time, :update_time, :expire_date, :register_date)", params!{
        "inscribe_num" => info.inscribe_num,
        "inscribe_id" => info.inscribe_id,
        "sat" => info.sat,
        "domain_name" => info.domain_name,
        "address" => info.address,
        "create_time" => info.create_time,
        "update_time" => info.update_time,
        "expire_date" => info.expire_date,
        "register_date" => info.register_date,
    });
    result
}

pub fn query_lastest_number() -> u64 {
    let mut conn = POOL.get_conn().unwrap();
    let sql = format!("SELECT * FROM domain_inscription_info order by create_time desc limit 1");
    info!("sql: {:?}", sql);
    let res = conn.query_map(sql, |(id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date)|{
        InscribeInfo { id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date }
    }).unwrap();
    info!("res: {:?}", res);
    if res.len() == 0 {
        START_INSCRIPTION_NUMBER
    }else {
        res[0].inscribe_num
    }
}

pub fn update_inscribe_info(dom_name: &str, owner_address: &str) -> Result<()>{
    let mut conn = POOL.get_conn().unwrap();
    let sql = format!("UPDATE domain_inscription_info SET address = '{}', update_time = {} WHERE domain_name = '{}'", owner_address, get_now_time(), dom_name);
    info!("sql: {}", sql);
    let x = conn.query_drop(sql);
    info!("update_inscribe_info: {:?}", x);
    Ok(())
}

pub fn query_all() -> Vec<InscribeInfo> {
    let mut conn = POOL.get_conn().unwrap();
    let sql = format!("SELECT * FROM domain_inscription_info");
    info!("sql: {:?}", sql);
    let res = conn.query_map(sql, |(id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date)|{
        InscribeInfo { id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date }
    }).unwrap();
    res
}

pub fn delete_from_id(id: u64) -> Result<()>{
    let mut conn = POOL.get_conn().unwrap();
    let sql = format!("delete from domain_inscription_info where id = {}", id);
    info!("sql: {}", sql);
    let x = conn.query_drop(sql);
    info!("delete_from_id: {:?}", x);
    Ok(())
}

pub fn query_by_domain(domain_name: &str) -> Vec<InscribeInfo> {
    let mut conn = POOL.get_conn().unwrap();
    let sql = format!("SELECT * FROM domain_inscription_info where domain_name = '{}' order by inscribe_num asc limit 1", domain_name);
    info!("sql: {:?}", sql);
    let res = conn.query_map(sql, |(id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date)|{
        InscribeInfo { id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date }
    }).unwrap();
    res
}

pub fn query_by_address(address: &str) -> Vec<InscribeInfo> {
    let mut conn = POOL.get_conn().unwrap();
    let sql = format!("SELECT * FROM domain_inscription_info where address = '{}'", address);
    info!("sql: {:?}", sql);
    let res = conn.query_map(sql, |(id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date)|{
        InscribeInfo { id, inscribe_num, inscribe_id, sat, domain_name, address, create_time, update_time, expire_date, register_date }
    }).unwrap();
    res
}