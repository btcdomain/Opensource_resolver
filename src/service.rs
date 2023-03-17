use std::process::Command;
use log::{warn};
use crate::{entity::InscribeContent};

pub fn get_inscribe_by_number(number: u64) -> Option<InscribeContent> {
    let output = Command::new("ord")
                .arg("find-number")
                .arg(number.to_string()).output().unwrap();
            
    if output.status.success() {
        let resp = serde_json::from_slice(&output.stdout);
        if resp.is_ok() {
            Some(resp.unwrap())
        }else {
            None
        }

    }else {
        warn!("get_inscribe_by_number failed number: {}, output: {:?}", number, String::from_utf8(output.stderr));
        None
    }
}