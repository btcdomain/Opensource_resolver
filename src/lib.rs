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
    
    info!("start btcdomain resolver");
    let database_url = get_database_url();
    info!("database: {}", database_url);
    if database_url == "None" {
        panic!("Database not config! please config database like 'export database=mysql://root:123456@localhost:3306/domain_inscription_data'")
    }
    
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any);
    let router = Router::new()
        .route("/open_api/domain/:domain", get(resolve_domain))
        .route("/open_api/domain_detail/:domain", get(resolve_detail_domain))
        .route("/open_api/address/:address", get(resolve_address))
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
    info!("resolve_domain: {:?}", domain);
    let query_result = query_by_domain(&domain);
    let address = if query_result.len() == 1 {
        let info = &query_result[0];
        let (check, code, addr) = check_inscription(info.inscribe_num, info.id, &info.address);
        if check.is_some() {
            addr
        }else {
            if code == ERROR_1 {
                let _ = delete_from_id(info.id);
            }
            String::new()
        }
    }else {
        String::new()
    };
    let resp = Json(InscribeResponse {
        code: 0,
        data: address,
        message: String::new()
    });
    resp.into_response()
}

async fn resolve_detail_domain(Path(domain): Path<String>) -> Response {
    let query_result = query_by_domain(&domain);
    let data: Option<InscribeInfoResp> = if query_result.len() == 1 {
        let info = &query_result[0];
        let (proof, code, addr) = check_inscription(info.inscribe_num, info.id, &info.address);
        if proof.is_some() {
            let domain = info.domain_name.clone();
            Some(InscribeInfoResp {
                inscribe_num: info.inscribe_num,
                inscribe_id: info.inscribe_id.clone(),
                domain_name: domain.clone(),
                address: addr,
                update_time: info.update_time,
                expire_date: info.expire_date,
                register_date: info.register_date,
                proof: proof.unwrap(),
                img_url: format!("{}/{}.jpeg", DEFAULT_IMG_URL, &domain[0..domain.len() - 4])
            })
        }else {
            if code == ERROR_1 {
                let _ = delete_from_id(info.id);
            }
            None
        }
    }else {
        None
    };
    let resp = Json(InscribeResponse {
        code: 0,
        data: data,
        message: String::new()
    });
    resp.into_response()
}

async fn resolve_address(Path(address): Path<String>) -> Response {
    let query_result = query_by_address(&address);
    let mut resp_data = Vec::new();
    for info in query_result.iter() {
        let (proof, code, addr) = check_inscription(info.inscribe_num, info.id, &info.address);
        if proof.is_some() {
            let domain = info.domain_name.clone();
            resp_data.push(InscribeInfoResp {
                inscribe_num: info.inscribe_num,
                inscribe_id: info.inscribe_id.clone(),
                domain_name: domain.clone(),
                address: addr,
                update_time: info.update_time,
                expire_date: info.expire_date,
                register_date: info.register_date,
                proof: proof.unwrap(),
                img_url: format!("{}/{}.jpeg", DEFAULT_IMG_URL, &domain[0..domain.len() - 4])
            });
        }else {
            if code == ERROR_1 {
                let _ = delete_from_id(info.id);
            }
        }
    }
    let resp = Json(InscribeResponse {
        code: 0,
        data: resp_data,
        message: String::new()
    });
    resp.into_response()
}

fn check_inscription(number: u64, id: u64, address: &str) -> (Option<Vec<u8>>, i32, String) {
    let (inscribe_result, code) = get_inscribe_by_number(number);
    if code == SUCCESS {
        if inscribe_result.is_some() {
            let content = inscribe_result.unwrap();
            let content_data = content.content;
            let address_online = content.address;
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
                        return (None, ERROR_1, String::new());
                    }

                    let sign_info = InscribeSignData{
                        name: domain_name.clone(),
                        first_owner: inscribe_data.first_owner,
                        create_date: inscribe_data.create_date,
                        register_date: inscribe_data.register_date,
                        expire_date: inscribe_data.expire_date
                    };
                    let sign_data = serde_json::to_vec(&sign_info).unwrap();
                    let verify_data = VerifyData {
                        data: sign_data,
                        signature: inscribe_data.sig
                    };

                    let proof = generate_proof(&verify_data, &domain_name);
                    if proof.is_some() {
                        if address == address_online {
                            let _ = update_inscribe_info_update_time(id);
                        }else {
                            let _ = update_inscribe_info(id, &address_online);
                        }
                        return (proof, SUCCESS, address_online);
                    }else {
                        return (None, ERROR_1, String::new());
                    }                  
                }else {
                    return (None, ERROR_1, String::new());
                }
            }
        }
    }else {
        return (None, code, String::new());
    }
    return (None, code, String::new());
}