use axum::{
    response::{Response, IntoResponse, Json},
    extract::Path,
    routing::{post, delete, get},
    http::StatusCode,
    Router
};

use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::tweet::{CreateTweetRequest, Tweet};

pub fn create_route() -> Router {
    Router::new()
        .route("/tweet", post(create_tweet))
        .route("/tweet/:id", delete(delete_tweet))
        .route("/tweet/:id", get(get_tweet))
}

pub async fn create_tweet(Json(body) :  Json<CreateTweetRequest> ) -> Response {
    todo!();
} 

pub async fn delete_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "DELETE", "id": id})))
} 
 

pub async fn get_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "GET", "id": id})))
}
