use crate::{get_inscribe_by_number, SUCCESS, ERROR_1, InscribeData, get_now_time, InscribeSignData, VerifyData, generate_proof, update_info_time, update_info_address};
use rocket::log::{info_ as info, warn_ as warn};

pub fn check_inscription(number: i64, id: i64, address: &str) -> (Option<Vec<u8>>, i32, String) {
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
                            let _ = update_info_time(id);
                        }else {
                            let _ = update_info_address(id, &address_online);
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
