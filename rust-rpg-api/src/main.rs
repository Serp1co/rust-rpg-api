use actix_web::{App, HttpServer};

pub mod apis;
pub mod configs;
pub mod data_models;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::data_models::{rest_models, user};
use apis::manage;
use apis::services::{echo, hello};
use configs::config::route_config;
use configs::config::ssl_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            manage::new_user_player_sheet
        ),
        components(
            schemas(
                user::Player, user::PlayerSheet, user::RaceSheet, 
                user::PlayerStatsPrimary, user::PlayerStatsDerived, 
                rest_models::NewPlayerSheet
            ),
            responses(
                user::Player, user::PlayerSheet, 
                user::RaceSheet, user::PlayerStatsPrimary, 
                user::PlayerStatsDerived
            ),
        ),
        tags(
            (name = "manage", description = "player management endpoints.")
        ),
    )]
    struct ApiDocPlayerManager;

    HttpServer::new(move || {
        App::new()
            .configure(route_config)
            .service(hello)
            .service(echo)
            .service(manage::new_user_player_sheet)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDocPlayerManager::openapi()),
            )
    })
    .bind_openssl(("0.0.0.0", 8083), ssl_config())?
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
