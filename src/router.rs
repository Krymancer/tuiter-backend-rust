use axum::Router;

use crate::routes::user;
use crate::routes::tweet;
use crate::routes::like;

use tower_http::cors::{CorsLayer, Any};


pub fn get_router() -> Router {
    let user_router = user::create_route();
    let tweet_router = tweet::create_route();
    let like_router = like::create_route();

    Router::new()
        .merge(user_router)
        .merge(tweet_router)
        .merge(like_router)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
}

