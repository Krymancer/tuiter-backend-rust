use axum::{response::Json, http::StatusCode, extract::Path};
use axum::{routing::{post, delete, get}, Router};
use serde_json::{json, Value};

use crate::models::tweet::{CreateTweetRequest};

pub fn create_route() -> Router {
    Router::new()
        .route("/tweet", post(create_tweet))
        .route("/tweet/:id", delete(delete_tweet))
        .route("/tweet/:id", get(get_tweet))
}


pub async fn create_tweet(Json(body) :  Json<CreateTweetRequest> ) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet", "method": "POST", "body": body}))) 
} 
 

pub async fn delete_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "DELETE", "id": id})))
} 
 

pub async fn get_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "GET", "id": id})))
}
