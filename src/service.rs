use std::process::Command;
use log::{info, warn};
use crate::{entity::InscribeContent, get_network};

pub fn get_inscribe_by_number(number: u64) -> Option<InscribeContent> {
    let network = get_network();
    let output = if network.len() == 0 {
        Command::new("ord")
                .arg("find-number")
                .arg(number.to_string()).output().unwrap()
    }else {
        Command::new("ord")
                .arg(network)
                .arg("find-number")
                .arg(number.to_string()).output().unwrap()
    };
    if output.status.success() {
        let resp = serde_json::from_slice(&output.stdout);
        if resp.is_ok() {
            // info!("get_inscribe_by_number: {:?}", resp);
            Some(resp.unwrap())
        }else {
            None
        }

    }else {
        warn!("get_inscribe_by_number failed number: {}, output: {:?}", number, String::from_utf8(output.stderr));
        None
    }
}