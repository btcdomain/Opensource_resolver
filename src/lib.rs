use log::info;
use axum::{Router};
use tower_http::cors::{CorsLayer, Any};
use std::{net::SocketAddr};
use axum::routing::{get, post};

mod db;
pub use db::*;
mod config;
pub use config::*;
mod service;
pub use service::*;
mod entity;
pub use entity::*;
mod sched;
use sched::*;
mod ecdsa;
pub use ecdsa::*;


pub async fn main() {
    
    info!("start inscribe ids");
    info!("network: {}", get_network());
    info!("database: {}", get_database_url());
    
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_origin(Any);
    let router = Router::new()
        // .route("/", get(|| async {"Hello World!!"}))
        // .route("/api/domain", post(get_balanace_handler))
        .layer(cors.clone());
        
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8088));
    info!("ord server start, listening on: {:?}", addr);
    
    tokio::spawn(sched_work());
    
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

}