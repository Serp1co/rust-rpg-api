use actix_web::{App, HttpServer};

pub mod apis;
pub mod configs;

use configs::configs::config;
use apis::services::{hello, echo};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config)
            .service(hello)
            .service(echo)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}