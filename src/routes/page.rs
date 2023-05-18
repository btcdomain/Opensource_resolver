use lazy_static::lazy_static;
use log::info;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket_okapi::openapi;
use std::collections::HashMap;
use rocket::response::Redirect;
use crate::{RequestParams, get_content_by_id_api, CacheInfo};

lazy_static!{
    pub static ref DEFAULT_FEE_MAP: HashMap<String, String> = HashMap::from([
        ("helloworld.btc".to_string(), "c0ff5c133d424706ca76c4f39f98a0f876b8e04fdf0fde5b5a0934252342da68i0".to_string()),
        ("ordinal.btc".to_string(), "484720360a3e2c0e64e89e2e0ee7112e0860d5a73d95e6b96c1f6552317b87afi0".to_string()),
        ("snakes.btc".to_string(), "9e18f15c62eec4f881b35da4be77b4d6023c098ac2140ab43bdfc446a5139945i0".to_string()),
    ]);
}

#[openapi(skip)]
#[get("/open_api/resolve_page")]
pub async fn resolve_page(req: RequestParams) -> Result<RawHtml<String>, Redirect> {
    info!("req: {:?}", req);
    let host = req.host;
    let domain = &host[0..host.len() - 5];
    info!("domain: {}", domain);
    let new_url = format!("https://app.btcdomains.io/#/?search={}", domain);
    let page_info = CacheInfo::find_by_key(&format!("cache_{}", domain));
    if page_info.is_ok() {
        let ins_id = page_info.unwrap().c_val;
        let content = get_content_by_id_api(&ins_id).await;
        return Ok(RawHtml(String::from_utf8(content.unwrap().content).unwrap()))
    }else {
        return Err(Redirect::to(new_url));
    }
    
    // let info = DomainInscriptionInfo::query_by_domain(domain);
    // info!("query domain: {}, info: {:?}", &domain, info);
    // let new_url = format!("https://app.btcdomains.io/#/?search={}", domain);
    // if info.is_ok() {
    //     let info = info.unwrap();
    //     if info.inscribe_id.len() == 0 {
    //         return Err(Redirect::to(new_url));
    //     }else {
    //         if domain == "helloworld.btc" || domain == "ordinal.btc" {
    //             let ins_id = DEFAULT_FEE_MAP.get(domain).unwrap();
    //             let content = get_inscribe_by_id_cmd(&ins_id).0;
    //             return Ok(RawHtml(String::from_utf8(content.unwrap().content).unwrap()))
    //         }else {
    //             return Err(Redirect::to(new_url));
    //         }
            // let inscribe_vec = query_uisat_address(&info.address).await;
            // let mut max_inscribe_number = 0;
            // let mut latest_inscribe_url_id = String::new();
            // for inscribe_data in inscribe_vec.result.iter() {
            //     let content_type = &inscribe_data.detail.content_type;
            //     let content = &inscribe_data.detail.content;
            //     let id = &inscribe_data.detail.id;
            //     info!("content_type: {}, content: {}, id: {}", content_type, content, &id);
            //     if content_type.starts_with("text") || content_type == "application/json" {
                    // let ins_id = DEFAULT_FEE_MAP.get(domain).unwrap();
                    // let (content_data, _) = get_inscribe_by_id_cmd(&ins_id);
                    // // info!("content_data: {:?}", content_data);
                    // if content_data.is_some() {
                    //     let content = serde_json::from_slice::<BtcDomainLink>(&content_data.unwrap().content);
                    //     if content.is_ok() {
                    //         info!("content: {:?}", &content);
                    //         let link = content.unwrap();
                    //         let link_domain = &link.domain;
                    //         if link_domain != domain {
                    //             return Err(Redirect::to(new_url));
                    //         }
                    //         let link_obj_ins_id = &link.obj_ins_id;
                    //         let block_check = BlackInfo::query_by_inscription_id(&link_obj_ins_id);
                    //         if block_check.is_ok() {
                    //             return Err(Redirect::to(new_url));
                    //         }
                    //         let link_public_key = &link.public_key;
                    //         let sign_info = BtcDomainLinkSign {
                    //             _type: link._type,
                    //             domain: link_domain.to_string(),
                    //             obj_ins_id: link_obj_ins_id.to_string(),
                    //             public_key: link_public_key.to_string()
                    //         };
                    //         let sign_data = serde_json::to_vec(&sign_info).unwrap();
                    //         if verify_compact(&sign_data, &link.sig, &link_public_key){
                    //             // if max_inscribe_number < inscribe_data.number {
                    //             //     max_inscribe_number = inscribe_data.number;
                    //             //     latest_inscribe_url_id = link_obj_ins_id.to_string();
                    //             // }
                    //             let content = get_inscribe_by_id_cmd(&ins_id).0;
                    //             return Ok(RawHtml(String::from_utf8(content.unwrap().content).unwrap()))
                    //         }else {
                    //             return Err(Redirect::to(new_url));
                    //         }
                    //     }else {
                    //         return Err(Redirect::to(new_url));
                    //     }
                    // }else {
                    //     return Err(Redirect::to(new_url));
                    // }
            //     }
            // }
            // info!("latest_inscribe_url_id: {}", latest_inscribe_url_id);
            // if latest_inscribe_url_id.len() > 0 {
            //     let content = get_inscribe_by_id_cmd(&latest_inscribe_url_id).0;
            //     return Ok(RawHtml(String::from_utf8(content.unwrap().content).unwrap()))
            // }else {
            //     return Err(Redirect::to(new_url));
            // }
    //     }
    // }else {
    //     return Err(Redirect::to(new_url));
    // }

}
