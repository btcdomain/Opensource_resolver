use log::warn;
use rocket::log::private::info;
use rocket::response::status;
use rocket::get;
use rocket::serde::json::{Value, json};
use rocket::log::info_;
use rocket_okapi::openapi;
use std::env::current_dir;
use std::fs::File;
use std::io::Read;
use crate::{get_inscribe_by_id};
use crate::{params::*, models::*, check_inscription, repo::DomainInscriptionInfo};


#[openapi(tag = "v1")]
#[get("/open_api/domain/<domain>")]
pub async fn resolve_domain(domain: String) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_domain: {:?}", domain);
    let query = DomainInscriptionInfo::query_by_domain(&domain);
    info!("query result: {:?}", query);
    let address = if query.is_ok() {
        let info = query.unwrap();
        let check_result = check_inscription(info.inscribe_num, info.id, &info.address).await;
        if check_result.is_some() {
            check_result.unwrap().address
        }else {
            String::new()
        }
    }else {
        String::new()
    };
    Ok(json!(InscribeResponse {
        code: SUCCESS,
        data: ResolveResp {
            address
        },
        message: String::new(),
    }))
}

#[openapi(tag = "v1")]
#[get("/open_api/domain_detail/<domain>")]
pub async fn resolve_detail_domain(domain: String) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_domain detail: {:?}", domain);
    let query = DomainInscriptionInfo::query_by_domain(&domain);
    info!("query result: {:?}", query);
    let data: Option<InscribeInfoResp> = if query.is_ok() {
        let info = query.unwrap();
        let check_result = check_inscription(info.inscribe_num, info.id, &info.address).await;
        if check_result.is_some() {
            check_result
        }else {
            None
        }
    }else {
        None
    };
    Ok(json!(InscribeResponse {
        code: SUCCESS,
        data,
        message: String::new(),
    }))
}

#[openapi(tag = "v1")]
#[get("/open_api/address/<address>")]
pub async fn resolve_address(address: String) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_address : {:?}", address);
    let query = DomainInscriptionInfo::query_by_address(&address);
    let mut resp_data = Vec::new();
    if query.is_ok() {
        let infos = query.unwrap();
        info_!("resolve_address query: {:?}", infos);
        for info in infos {
            let check_result = check_inscription(info.inscribe_num, info.id, &info.address).await;
            if check_result.is_some() {
                resp_data.push(check_result.unwrap());
            }
        }
    };
        
    Ok(json!(InscribeResponse {
        code: SUCCESS,
        data: resp_data,
        message: String::new(),
    }))
}





#[openapi(tag = "v1")]
#[get("/open_api/load_default_data")]
pub async fn load_default_data() -> Result<Value, status::Custom<Value>> {
    info_!("resolve_address");
    let mut file = File::open(&current_dir().unwrap().join("dom_txt")).unwrap();
    let mut data = String::new();
    let read_result = file.read_to_string(&mut data);
    info!("read_result: {:?}", read_result);
    let split_vec: Vec<&str> = data.split("\n").collect();
    for row in split_vec {
        let split_row: Vec<&str> = row.split("|").collect();
        let domain = split_row[0].trim();
        let ins_id = split_row[1].trim();
        info!("domain: {}, ins_id: {}", domain, ins_id);
        if ins_id.len() > 20 {
            let content = get_inscribe_by_id(ins_id).0;
            if content.is_some() {
                let content = content.unwrap();
                let format_data = serde_json::from_slice(&content.content);
                if format_data.is_ok() {
                    let inscribe_data: InscribeData = format_data.unwrap();
                    info!("inscribe data: {:?}", inscribe_data);
                    
                    let domain_name = inscribe_data.name;
                    let expire_date = inscribe_data.expire_date;
                    let now_date = get_now_time();
                    if expire_date < now_date {
                        warn!("domain: {}, is expired, now: {}, expire_time: {}", domain_name, now_date, expire_date);
                        continue;
                    }
                    let check = DomainInscriptionInfo::query_by_domain(&domain_name);
                    if check.is_ok() {
                        let info = check.unwrap();
                        let update_result = DomainInscriptionInfo::update_info_address(info.id, &content.output_address);
                        info!(": {:?}", update_result);
                    }else {
                        let info = DomainInscriptionInfo { 
                            id: 0,
                            inscribe_num: content.inscribe_num, 
                            inscribe_id: ins_id.to_string(), 
                            sat: 0, 
                            domain_name: domain_name.clone(), 
                            address: content.output_address,
                            create_time: get_now_time(),
                            update_time: get_now_time(),
                            expire_date: expire_date,
                            register_date: inscribe_data.register_date,
                        };
                        let insert_result = DomainInscriptionInfo::insert_inscribe_info(info);
                        info!("insert_result: {:?}", insert_result);
                        if insert_result.is_ok() {
                            
                        }else {
                            break;
                        }
                    }

                }
            }else {
                warn!("err: id: {}, {:?}", &ins_id, content);
            }
        }
            

    }
    Ok(json!(InscribeResponse {
        code: SUCCESS,
        data: String::new(),
        message: String::new(),
    }))
}