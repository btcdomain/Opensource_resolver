use btcdomain_resolve_rocket::{resolve_domain, resolve_detail_domain, resolve_address, sched_work};

use rocket::{routes, catchers, catch};
use rocket::serde::json::{Value, serde_json::json};

#[catch(404)]
async fn not_found_url() -> Value {
    json!("not found!")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    rocket::tokio::spawn(sched_work());

    rocket::build()
        .mount("/open_api", routes![
            resolve_domain,
            resolve_detail_domain,
            resolve_address
        ])
        .register("/", catchers!(not_found_url))
        .launch().await?;


    Ok(())
}
