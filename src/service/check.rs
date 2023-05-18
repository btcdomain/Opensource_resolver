use crate::{SUCCESS, ERROR_1, InscribeData, get_now_time, InscribeSignData, VerifyData, generate_proof, verify, PUBLIC_KEY, DomainInscriptionInfo, get_content_by_number_api, InscribeInfoResp};
use rocket::log::{info_ as info, warn_ as warn};

pub async fn check_inscription(number: i64, id: i64, address: &str) -> Option<InscribeInfoResp> {
    let inscribe_result = get_content_by_number_api(number).await;
    if inscribe_result.is_ok() {
        let content = inscribe_result.unwrap();
        let content_data = content.content;
        let address_online = content.address;
        let inscribe_id = content.inscribe_id;
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
                    return None;
                }

                let sign_info = InscribeSignData{
                    name: domain_name.clone(),
                    first_owner: inscribe_data.first_owner,
                    create_date: inscribe_data.create_date,
                    register_date: inscribe_data.register_date,
                    expire_date: inscribe_data.expire_date
                };
                let sign_data = serde_json::to_vec(&sign_info).unwrap();
                
                if verify(&sign_data, &inscribe_data.sig, PUBLIC_KEY){
                    if address == address_online {
                        let _ = DomainInscriptionInfo::update_info_time(id);
                    }else {
                        let _ = DomainInscriptionInfo::update_info_address(id, &address_online);
                    }
                    return Some(InscribeInfoResp {
                        inscribe_num: number,
                        inscribe_id,
                        domain_name,
                        address: address_online,
                        create_date: inscribe_data.create_date,
                        expire_date: inscribe_data.expire_date,
                        register_date: inscribe_data.register_date,
                        img_url: inscribe_data.img_url,
                    });
                }else {
                    return None;
                }                  
            }else {
                return None;
            }
        }
    }
    return None;
}
