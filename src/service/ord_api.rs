use crate::{ InscribeApiContent, InscriptionTotal, InscribeContent};
use std::{sync::{Arc, RwLock}, collections::VecDeque};


lazy_static::lazy_static! {
    pub static ref CUR_HTTP_INDEX: Arc<RwLock<VecDeque<usize>>> = Arc::new(RwLock::new(VecDeque::new()));
}

fn get_port() -> i32{
    let size = CUR_HTTP_INDEX.read().unwrap().len();
    CUR_HTTP_INDEX.write().unwrap().push_front(size + 1);
    if size + 1 % 2 == 0 {
        if size == 2000 {
            CUR_HTTP_INDEX.write().unwrap().clear();
        }
        8091
    }else {
        8092
    }
}

pub async fn get_content_by_id_api(id: &str) -> Result<InscribeContent, reqwest::Error> {
    reqwest::get(format!("http://127.0.0.1:{}/api/inscription/{}", get_port(), id))
        .await?
        .json::<InscribeContent>()
        .await
}

pub async fn get_content_by_number_api(number: i64) -> Result<InscribeContent, reqwest::Error> {
    reqwest::get(format!("http://127.0.0.1:{}/api/inscription_number/{}", get_port(), number))
        .await?
        .json::<InscribeContent>()
        .await
}

pub async fn get_inscribe_by_number_api(number: i64) -> Result<InscribeApiContent, reqwest::Error> {
    reqwest::get(format!("http://127.0.0.1:{}/api/inscription_all/{}", get_port(), number))
        .await?
        .json::<InscribeApiContent>()
        .await
}

pub async fn get_inscribe_total_api() -> Result<InscriptionTotal, reqwest::Error> {
    reqwest::get(format!("http://127.0.0.1:{}/api/inscription_total", get_port()))
        .await?
        .json::<InscriptionTotal>()
        .await

}