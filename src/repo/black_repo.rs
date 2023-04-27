use diesel::{Insertable, AsChangeset, Queryable, QueryDsl, ExpressionMethods, RunQueryDsl, QueryResult};
use rocket::{serde::{Serialize, Deserialize}};
use crate::{schemas::black_info, POOL, repo::*, black_info::dsl::*};


#[derive(Deserialize, Serialize, Insertable, AsChangeset, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = black_info)]
pub struct BlackInfo {
    pub id: i64,
    pub inscribe_num: i64,
    pub inscribe_id: String,
    pub create_time: i64,
    pub update_time: i64
}

impl BlackInfo {
    pub fn insert(info: BlackInfo) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        diesel::insert_into(black_info::table)
            .values(&info)
            .execute(&mut conn)
    }

    pub fn query_by_inscription_id(in_id: &str) -> QueryResult<BlackInfo> {
        let mut conn = POOL.get().unwrap();
        black_info.filter(inscribe_id.eq(in_id)).get_result(&mut conn)
    }
}