use axum::{
    response::{Response, IntoResponse, Json},
    extract::{Path, State},
    routing::{post, delete, get},
    http::StatusCode,
    Router, Extension,
};

use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    models::tweet::{CreateTweetRequest, Tweet}, 
    extractor::AuthUser, 
    router::ApiContext
};

pub fn create_route() -> Router<ApiContext> {
    Router::new()
        .route("/tweet", post(create_tweet))
        .route("/tweet/:id", get(get_tweet).delete(delete_tweet))
}

pub async fn create_tweet(
    auth_user: AuthUser,
    context: State<ApiContext>,
    Json(request): Json<CreateTweetRequest>
) -> Response {

    (StatusCode::OK, Json(json!({"message": "ok"}))).into_response()
} 

pub async fn delete_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "DELETE", "id": id})))
} 
 

pub async fn get_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "GET", "id": id})))
}
