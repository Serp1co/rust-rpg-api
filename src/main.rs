use actix_web::{App, HttpServer};

pub mod apis;
pub mod configs;

use configs::configs::route_config;
use configs::configs::ssl_config;
use apis::services::{hello, echo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(route_config)
            .service(hello)
            .service(echo)
    })
    .bind_openssl(("0.0.0.0", 8083), ssl_config())?
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}