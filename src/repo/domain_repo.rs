use diesel::{Insertable, AsChangeset, Queryable, QueryDsl, ExpressionMethods, RunQueryDsl, QueryResult};
use rocket::{serde::{Serialize, Deserialize}};
use crate::{schemas::domain_inscription_info, POOL, get_now_time, repo::*, domain_inscription_info::dsl::*, START_INSCRIPTION_NUMBER};


#[derive(Deserialize, Serialize, Insertable, AsChangeset, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = domain_inscription_info)]
pub struct DomainInscriptionInfo {
    pub id: i64,
    pub inscribe_num: i64,
    pub inscribe_id: String,
    pub sat: i64,
    pub domain_name: String,
    pub address: String,
    pub create_time: i64,
    pub update_time: i64,
    pub expire_date: i64,
    pub register_date: i64,
}

impl DomainInscriptionInfo {
    pub fn insert_inscribe_info(info: DomainInscriptionInfo) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        diesel::insert_into(domain_inscription_info::table)
            .values(&info)
            .execute(&mut conn)
    
    }
    
    pub fn query_by_domain(domain: &str) -> QueryResult<DomainInscriptionInfo> {
        let mut conn = POOL.get().unwrap();
        domain_inscription_info.filter(domain_name.eq(domain)).get_result(&mut conn)
    }
    
    pub fn query_by_number(num: i64) -> QueryResult<DomainInscriptionInfo> {
        let mut conn = POOL.get().unwrap();
        domain_inscription_info.filter(inscribe_num.eq(num)).get_result(&mut conn)
    }
    
    pub fn query_by_address(addr: &str) -> QueryResult<Vec<DomainInscriptionInfo>> {
        let mut conn = POOL.get().unwrap();
        domain_inscription_info.filter(address.eq(addr)).get_results(&mut conn)
    }
    
    pub fn query_lastest_number() -> QueryResult<i64> {
        let mut conn = POOL.get().unwrap();
        let result = domain_inscription_info::table.order(domain_inscription_info::id.desc()).limit(1).load::<DomainInscriptionInfo>(&mut conn).ok();
        info!("query_lastest_number: {:?}", result);
        if result.is_some() {
            Ok(result.unwrap()[0].inscribe_num)
        }else {
            Ok(START_INSCRIPTION_NUMBER)
        }
    }
    
    pub fn query_all() -> QueryResult<Vec<DomainInscriptionInfo>> { 
        let mut conn = POOL.get().unwrap();
        domain_inscription_info::table.order(domain_inscription_info::id.desc()).load::<DomainInscriptionInfo>(&mut conn)
    }
    
    pub fn update_info_time(data_id: i64) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        diesel::update(domain_inscription_info::table.find(data_id))
            .set(domain_inscription_info::update_time.eq(get_now_time()))
            .execute(&mut conn)
    }
    
    pub fn update_info_address(data_id: i64, new_address: &str) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        diesel::update(domain_inscription_info::table.find(data_id))
            .set(
                (
                    domain_inscription_info::update_time.eq(get_now_time()),
                    domain_inscription_info::address.eq(new_address)
                )
            )
            .execute(&mut conn)
    }
    
    pub fn delete_info(data_id: i64) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        diesel::delete(domain_inscription_info::table.find(data_id)).execute(&mut conn)
    }
}