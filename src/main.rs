use btcdomain_resolver::*;
use rocket_okapi::{openapi_get_routes, swagger_ui::*};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    rocket::tokio::spawn(sched_work());

    let _ = rocket::build()
        .mount("/", 
            openapi_get_routes![
                resolve_domain,
                resolve_detail_domain,
                resolve_address,
                resolve_page
            ]
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .attach(get_cors())
        .launch().await?;
    Ok(())
}
