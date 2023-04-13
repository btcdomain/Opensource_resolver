use btcdomain_resolve::{resolve_domain, resolve_detail_domain};

use rocket::{routes, catchers, catch};
use rocket::serde::json::{Value, serde_json::json};
use btcdomain_resolve::DbConn;

#[catch(404)]
async fn not_found_url() -> Value {
    json!("not found!")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount("/open_api", routes![
            resolve_domain,
            resolve_detail_domain
        ])
        .register("/", catchers!(not_found_url))
        .attach(DbConn::fairing())
        .launch().await?;
    Ok(())
}
