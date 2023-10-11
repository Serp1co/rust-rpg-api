use actix_web::{
    get, post,
    web::{Json, Path},
    HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use crate::data_models::rest_models::{NewPlayerSheet, UserLogin, UserSignup};
use crate::data_models::user::{Player, PlayerSheet};

#[derive(Serialize, Clone)]
struct UserSignupResponse {
    friend: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserSearchRequest {
    filter: String,
}

#[post("/users/login")]
async fn login(req_body: Json<UserLogin>) -> impl Responder {
    HttpResponse::NotImplemented().json(req_body)
}

#[post("/users/signup")]
async fn signup(req_body: Json<UserSignup>) -> impl Responder {
    let response = UserSignupResponse {
        friend: req_body.username.clone(),
    };
    HttpResponse::NotImplemented().json(response)
}

// semantics for get with request body changed
#[post("/users")]
async fn search_users(req_body: Json<UserSearchRequest>) -> impl Responder {
    HttpResponse::NotImplemented().json(req_body)
}

#[get("/users/{username}")]
async fn get_user(username: Path<String>) -> impl Responder {
    HttpResponse::NotImplemented().json(username.clone())
}

#[post("/users/{username}/players")]
async fn get_user_players_list(path: Path<String>) -> impl Responder {
    let username = path.into_inner();
    let mut response: Vec<String> = Vec::new();
    response.push(username);
    HttpResponse::NotImplemented().json(response)
}

#[utoipa::path(
    post, 
    path = "/users/{username}/player", 
    responses(
        (status = 200, description = "New Player Sheet created", body = Vec<Player>),
        (status = 400, description = "Error creating new Player")
    ),
    params(
        ("username" = String, Path, description = "The user to add the new player sheet"),
    )
)]
#[post("/users/{username}/player")]
async fn new_user_player_sheet(
    path: Path<String>,
    req_body: Json<NewPlayerSheet>,
) -> impl Responder {
    let username = path.into_inner();
    let sheet = PlayerSheet::new(
        username.clone(),
        req_body.surname.clone(),
        req_body.race.clone(),
        0,
        0,
        req_body.stats_primary.clone(),
        None,
    );
    let player: Player = Player {
        player_sheet: sheet,
        user: username,
    };
    let response: Vec<Player> = Vec::from([player]);
    HttpResponse::NotImplemented().json(response)
}
