use log::info;
use rocket::get;
use rocket::response::content::RawHtml;
use rocket_okapi::openapi;
use rocket::response::Redirect;
use crate::{RequestParams, query_uisat_address, query_content_by_id, query_by_url, BtcDomainLink, BtcDomainLinkSign, verify_compact, repo::DomainInscriptionInfo, BlackInfo};

#[openapi(skip)]
#[get("/open_api/resolve_page")]
pub async fn resolve_page(req: RequestParams) -> Result<RawHtml<String>, Redirect> {
    info!("req: {:?}", req);
    let host = req.host;
    let domain = &host[0..host.len() - 5];
    info!("domain: {}", domain);
    
    let info = DomainInscriptionInfo::query_by_domain(domain);
    info!("query domain: {}, info: {:?}", &domain, info);
    let new_url = format!("https://app.btcdomains.io/#/?search={}", domain);
    if info.is_ok() {
        let info = info.unwrap();
        if info.inscribe_id.len() == 0 {
            return Err(Redirect::to(new_url));
        }else {
            let inscribe_vec = query_uisat_address(&info.address).await;
            let mut max_inscribe_number = 0;
            let mut latest_inscribe_url_id = String::new();
            for inscribe_data in inscribe_vec.result.iter() {
                let content_type = &inscribe_data.detail.content_type;
                let content = &inscribe_data.detail.content;
                info!("content_type: {}, content: {}", content_type, content);
                if content_type == "application/json" {
                    let content_data = query_by_url(&content).await;
                    info!("content_data: {:?}", content_data);
                    if content_data.is_some() {
                        let content = serde_json::from_str::<BtcDomainLink>(&content_data.unwrap());
                        if content.is_ok() {
                            let link = content.unwrap();
                            let link_domain = &link.domain;
                            if link_domain != domain {
                                continue;
                            }
                            let link_obj_ins_id = &link.obj_ins_id;
                            let block_check = BlackInfo::query_by_inscription_id(&link_obj_ins_id);
                            if block_check.is_ok() {
                                continue;
                            }
                            let link_public_key = &link.public_key;
                            let sign_info = BtcDomainLinkSign {
                                _type: link._type,
                                domain: link_domain.to_string(),
                                obj_ins_id: link_obj_ins_id.to_string(),
                                public_key: link_public_key.to_string()
                            };
                            let sign_data = serde_json::to_vec(&sign_info).unwrap();
                            if verify_compact(&sign_data, &link.sig, &link_public_key){
                                if max_inscribe_number < inscribe_data.number {
                                    max_inscribe_number = inscribe_data.number;
                                    latest_inscribe_url_id = link_obj_ins_id.to_string();
                                }
                            }
                        }
                    }
                }
            }
            info!("latest_inscribe_url_id: {}", latest_inscribe_url_id);
            if latest_inscribe_url_id.len() > 0 {
                let content = query_content_by_id(&latest_inscribe_url_id).await;
                return Ok(RawHtml(content.unwrap()))
            }else {
                return Err(Redirect::to(new_url));
            }

        }
    }else {
        return Err(Redirect::to(new_url));
    }

}
