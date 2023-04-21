// use diesel::{QueryResult, query_dsl::methods::{LimitDsl, FindDsl, OrderDsl, SelectDsl, FilterDsl}, RunQueryDsl, ExpressionMethods};
use rocket::http::Status;
use rocket::response::status;
use rocket::{get};
use rocket::serde::json::{Value, json};
use rocket::log::info_;
use crate::{params::*, query_by_domain, models::*, check_inscription, query_by_address};


#[get("/domain/<domain>")]
pub async fn resolve_domain(domain: String) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_domain: {:?}", domain);
    let query = query_by_domain(&domain);
    let addr_info = if query.is_ok() {
        
        let info = query.unwrap();
        let (proof, _, _) = check_inscription(info.inscribe_num, info.id, &info.address);
        if proof.is_some() {
            let name = &domain[0..domain.len() - 4];
            ResolveResp {
                proof: vec![],
                address: info.address,
                proof_url: format!("{}/{}.bin", get_proof_file(), name)
            }
        }else {
            ResolveResp {
                proof: vec![],
                address: String::new(),
                proof_url: String::new(),
            }
        }
    }else {
        ResolveResp {
            proof: vec![],
            address: String::new(),
            proof_url: String::new(),
        }
    };
    Ok(json!(InscribeResponse {
        code: SUCCESS,
        data: addr_info,
        message: String::new(),
    }))
}

#[get("/domain_detail/<domain>")]
pub async fn resolve_detail_domain(domain: String) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_domain detail: {:?}", domain);
    let query = query_by_domain(&domain);
    let data: Option<InscribeInfoResp> = if query.is_ok() {
        let info = query.unwrap();
        let (proof, _, addr) = check_inscription(info.inscribe_num, info.id, &info.address);
        if proof.is_some() {
            let domain = info.domain_name.clone();
            let name = &domain[0..domain.len() - 4];
            Some(InscribeInfoResp {
                inscribe_num: info.inscribe_num,
                inscribe_id: info.inscribe_id.clone(),
                domain_name: domain.clone(),
                address: addr,
                update_time: info.update_time,
                expire_date: info.expire_date,
                register_date: info.register_date,
                proof: vec![],
                img_url: format!("{}/{}.jpeg", DEFAULT_IMG_URL, name),
                proof_url: format!("{}/{}.bin", get_proof_file(), name)
            })
        }else {
            // if code == ERROR_1 {
            //     let _ = delete_from_id(info.id);
            // }
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


#[get("/address/<address>")]
pub async fn resolve_address(address: String) -> Result<Value, status::Custom<Value>> {
    info_!("resolve_address : {:?}", address);
    let query = query_by_address(&address);
    let mut resp_data = Vec::new();
    if query.is_ok() {
        let infos = query.unwrap();
        for info in infos {
            let (proof, _, addr) = check_inscription(info.inscribe_num, info.id, &info.address);
            if proof.is_some() {
                let domain = info.domain_name.clone();
                let name = &domain[0..domain.len() - 4];
                resp_data.push(InscribeInfoResp {
                    inscribe_num: info.inscribe_num,
                    inscribe_id: info.inscribe_id.clone(),
                    domain_name: domain.clone(),
                    address: addr,
                    update_time: info.update_time,
                    expire_date: info.expire_date,
                    register_date: info.register_date,
                    proof: vec![],
                    img_url: format!("{}/{}.jpeg", DEFAULT_IMG_URL, name),
                    proof_url: format!("{}/{}.bin", get_proof_file(), name)
                })
            }
        }
    };
        
    Ok(json!(InscribeResponse {
        code: SUCCESS,
        data: resp_data,
        message: String::new(),
    }))
}


