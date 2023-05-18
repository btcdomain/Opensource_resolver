use rocket::serde::{Deserialize};
use diesel::{Insertable, AsChangeset, Queryable, RunQueryDsl, QueryResult, QueryDsl, ExpressionMethods};
use crate::{POOL, cache_info::dsl::*, schemas::cache_info};
use chrono::NaiveDateTime;


#[derive(Deserialize, Debug, Insertable, AsChangeset, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = cache_info)]
pub struct CacheInfo{
    pub id: i64,
    pub c_key: String,
    pub c_val: String,
    pub create_time: NaiveDateTime,
}

impl CacheInfo {
    pub fn insert(info: CacheInfo) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        let result = diesel::insert_into(cache_info::table)
            .values(&info)
            .execute(&mut conn)?;
        Ok(result)
    }

    pub fn find_by_key(key: &str) -> QueryResult<CacheInfo> {
        let mut conn = POOL.get().unwrap();
        let result = cache_info.filter(c_key.eq(key)).get_result(&mut conn)?;
        Ok(result)
    }

    pub fn update_cache(key: &str, value: &str) -> QueryResult<usize> {
        let mut conn = POOL.get().unwrap();
        let result = diesel::update(cache_info::table.filter(c_key.eq(key)))
            .set((
                cache_info::c_val.eq(value),
            ))
            .execute(&mut conn)?;
        Ok(result)
    }

}