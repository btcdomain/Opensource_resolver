use std::process::Command;
use log::{warn};
use crate::{entity::InscribeContent, ERROR_1, ERROR_2, SUCCESS};

pub fn get_inscribe_by_number(number: u64) -> (Option<InscribeContent>, i32) {
    let output = Command::new("ord")
                .arg("find-number")
                .arg(number.to_string()).output().unwrap();
            
    if output.status.success() {
        let resp = serde_json::from_slice(&output.stdout);
        if resp.is_ok() {
            (Some(resp.unwrap()), SUCCESS)
        }else {
            (None, ERROR_1)
        }

    }else {
        warn!("get_inscribe_by_number failed number: {}, output: {:?}", number, String::from_utf8(output.stderr));
        (None, ERROR_2)
    }
}