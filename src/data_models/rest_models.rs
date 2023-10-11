use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::user::{PlayerStatsPrimary, RaceSheet};

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct NewPlayerSheet {
    pub name: String,
    pub surname: String,
    pub race: RaceSheet,
    pub stats_primary: PlayerStatsPrimary,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct UserSignup {
    pub username: String,
    pub password: String,
    pub email: String,
}
