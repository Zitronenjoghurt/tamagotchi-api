use axum::Router;
use std::io;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

pub mod authentication;
mod database;
mod docs;

pub mod entities {
    pub mod user;
}

pub mod models {
    pub mod response_models;
}

pub mod resources {
    pub mod ping;
}

#[derive(Clone)]
pub struct AppState {
    database: database::DB,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = database::setup().await.expect("Failed to set up MongoDB.");

    let app_state = AppState { database: db };

    let app = Router::<AppState>::new()
        .nest("/", resources::ping::router())
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", docs::ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", docs::ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await
}
