use diesel::{QueryResult, query_dsl::methods::{LimitDsl, FindDsl, OrderDsl, SelectDsl, FilterDsl}, RunQueryDsl, ExpressionMethods};
use rocket::http::Status;
use rocket::response::status;
use rocket::{get};
use rocket::serde::json::{Value, json};
use rocket::log::info_;
use crate::{DbConn, DomainInscriptionInfo, ResolveResp};
use crate::domain_inscription_info::dsl::*;
use crate::params::*;

#[get("/domain/<domain>")]
pub async fn resolve_domain(domain: String, db: DbConn) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_domain: {:?}", domain);
    db.run(move |conn| {
        let result = domain_inscription_info.filter(domain_name.eq(domain.clone()))
            .first::<DomainInscriptionInfo>(conn)
            .ok();

        if result.is_some() {
            let info = result.unwrap();
            Ok(json!(ResolveResp {
                proof: vec![],
                address: info.address,
                proof_url: format!("{}/{}.bin", get_proof_file(), &domain)
            }))
        }else {
            Ok(json!(""))
        }
    }).await
}

#[get("/domain_detail/<domain>")]
pub async fn resolve_detail_domain(domain: String, db: DbConn) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_domain: {:?}", domain);
    db.run(move |conn| {
        let result = domain_inscription_info.filter(domain_name.eq(domain))
            .load::<DomainInscriptionInfo>(conn)
            .ok();

        if result.is_some() {
            Ok(json!(result))
        }else {
            Ok(json!(""))
        }
        
    }).await

}
