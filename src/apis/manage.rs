use actix_web::{web::Json, get, post, HttpResponse, Responder};
use serde::Serialize;
use oasgen::{OaSchema, Server, openapi};

use crate::data_models::user::{UserSignup, UserLogin, Player};

#[derive(Serialize, Clone)]
struct UserSignupResponse {
    friend: String,
}

#[post("/users/login")]
async fn login() -> impl Responder {
    HttpResponse::NotImplemented()
}

#[post("/users/signup")]
async fn signup(req_body: Json<UserSignup>) -> impl Responder {
    let response = Json(UserSignupResponse {
        friend: req_body.username.clone()
    });
    HttpResponse::NotImplemented().body(serde_json::to_string(&response).unwrap())
}