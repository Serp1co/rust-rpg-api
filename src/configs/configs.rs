use actix_web::{web, HttpResponse};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

// this function could be located in a different module
pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

// tls builder with nopass private key
pub fn ssl_config() -> SslAcceptorBuilder {
    let mut ssl_builder = SslAcceptor::
        mozilla_intermediate(SslMethod::tls_server())
        .unwrap();
    ssl_builder.set_private_key_file("nopass.pem", SslFiletype::PEM)
        .unwrap();
    ssl_builder.set_certificate_chain_file("cert.pem")
        .unwrap();
    return ssl_builder;
}