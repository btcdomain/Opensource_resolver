use log::{info, warn};
use axum::{Router, Json};
use axum::extract::Path;
use tower_http::cors::{CorsLayer, Any};
use std::{net::SocketAddr};
use axum::routing::{get};
use axum::response::{Response, IntoResponse};

mod db;
pub use db::*;
mod config;
pub use config::*;
mod service;
pub use service::*;
mod entity;
pub use entity::*;
mod sched;
use sched::*;
mod ecdsa;
pub use ecdsa::*;


pub async fn main() {
    
    info!("start inscribe ids");
    let database_url = get_database_url();
    info!("database: {}", database_url);
    if database_url == "None" {
        panic!("Database not config! please config database like 'export database=mysql://root:123456@localhost:3306/domain_inscription_data'")
    }
    
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any);
    let router = Router::new()
        .route("/api/domain/:domain", get(resolve_domain))
        .route("/api/address/:address", get(resolve_address))
        .layer(cors.clone());
        
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    info!("ord server start, listening on: {:?}", addr);
    
    tokio::spawn(sched_work());
    
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

}

async fn resolve_domain(Path(domain): Path<String>) -> Response {
    let query_result = query_by_domain(&domain);
    let mut resp_data = Vec::new();
    for info in query_result.iter() {
        let check = check_inscription(info.inscribe_num);
        if check {
            resp_data.push(info);
        }else {
            let _ = delete_from_id(info.id);
        }
    }
    let resp = Json(InscribeResponse {
        code: 0,
        data: resp_data,
        message: String::new()
    });
    resp.into_response()
}

async fn resolve_address(Path(address): Path<String>) -> Response {
    let query_result = query_by_address(&address);
    let mut resp_data = Vec::new();
    for info in query_result.iter() {
        let check = check_inscription(info.inscribe_num);
        if check {
            resp_data.push(info);
        }else {
            let _ = delete_from_id(info.id);
        }
    }
    let resp = Json(InscribeResponse {
        code: 0,
        data: query_result,
        message: String::new()
    });
    resp.into_response()
}

fn check_inscription(number: u64) -> bool {
    let inscribe_result = get_inscribe_by_number(number);
    if inscribe_result.is_some() {
        let content = inscribe_result.unwrap();
        let content_data = content.content;
        let length = content_data.len();
        if length > 350 && length < 500 {
            let format_data = serde_json::from_slice(&content_data);
            if format_data.is_ok() {
                let inscribe_data: InscribeData = format_data.unwrap();
                info!("inscribe data: {:?}", inscribe_data);
                
                let domain_name = inscribe_data.name;
                let expire_date = inscribe_data.expire_date;
                let now_date = get_now_time();
                if expire_date < now_date {
                    warn!("domain: {}, is expired, now: {}, expire_time: {}", domain_name, now_date, expire_date);
                    return false;
                }

                let sign_info = InscribeSignData{
                    name: domain_name.clone(),
                    first_owner: inscribe_data.first_owner,
                    create_date: inscribe_data.create_date,
                    register_date: inscribe_data.register_date,
                    expire_date: inscribe_data.expire_date
                };
                let sign_data = serde_json::to_vec(&sign_info).unwrap();
                if ecdsa::verify(&sign_data, &inscribe_data.sig) {
                    info!("ecds signature verify success");
                    return true;
                }else {
                    warn!("ecds signature verify failed");
                    return false;
                }
                
            }else {
                return false;
            }
        }
    }
    return false;
}