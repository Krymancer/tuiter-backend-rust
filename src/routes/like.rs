use axum::{response::Json, http::StatusCode, extract::Path};
use axum::{routing::{post, delete}, Router};
use serde_json::{json, Value};

use crate::router::ApiContext;

pub fn create_route() -> Router<ApiContext> {
    Router::new()
        .route("/like/:id", post(create_like))
        .route("/like/:id", delete(delete_like))
}

pub async fn create_like(Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/like", "method": "POST", "id": id}))) 
}

pub async fn delete_like(Path(id): Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/like/:id", "method": "DELETE", "id": id})))
}
