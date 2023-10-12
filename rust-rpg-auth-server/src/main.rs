use std::sync::Arc;

use serde::{Serialize, Deserialize};

pub struct User {
    pub uid: String,
    pub email: String,
    pub psw: String,
    pub role: String
}

#[derive(Deserialize)] 
pub struct LoginRequest {
    pub email: String,
    pub psw: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub email: String,
    pub psw: String
}

#[tokio::main]
async fn main() {
    // let users = Arc::new(init_users());
    let login_route = warp::path!("login")
}