use std::sync::Arc;
use anyhow::Context;
use axum::{Router, Extension};
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use tower::ServiceBuilder;
use sqlx::sqlite::SqlitePool;

use crate::config::Config;
use crate::routes::user;
use crate::routes::tweet;
use crate::routes::like;

#[derive (Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub database: SqlitePool,
}

pub fn get_router(state: ApiContext) -> Router {
    let user_router = user::create_route();
    let tweet_router = tweet::create_route();
    let like_router = like::create_route();

    Router::new()
        .merge(user_router)
        .merge(tweet_router)
        .merge(like_router)
        .with_state(state)
}

pub async fn serve(config: Config, database: SqlitePool) -> anyhow::Result<()> {
    let api_state = ApiContext {
        config: Arc::new(config),
        database,
    };

    let app = get_router(api_state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    axum::Server::bind(&"0.0.0.0:1234".parse()?)
        .serve(app.into_make_service())
        .await
        .context("Failed to start server")
}
