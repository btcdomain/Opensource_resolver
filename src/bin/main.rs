use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("service start");
    btcdomain_resolver::main().await;
}