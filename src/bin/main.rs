use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("service start");
    inscribe::main().await;
}