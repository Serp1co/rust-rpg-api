use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/long-post")]
pub async fn long_post(req_body: String) -> impl Responder {
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    HttpResponse::Ok().body(req_body)
}
