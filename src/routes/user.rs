use axum::{response::Json, http::StatusCode};
use axum::{routing::post, Router};
use serde_json::{json, Value};

use crate::models::user::{User, CreateUserRequest, AuthenticateUserRequest};

use crate::repository::user::{insert_user, get_user_by_username};

pub fn create_route() -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/auth", post(authenticate_user))
}

async fn create_user(Json(body): Json<CreateUserRequest>) -> Result<Json<Value>, StatusCode> {
    let user = User::new(body.username, body.password);

    insert_user(&user).await;

    let response = Json(json!({
        "user": user
    }));

    Ok(response)
}

async fn authenticate_user(Json(body) : Json<AuthenticateUserRequest>) -> Result<Json<Value>, StatusCode> {
    
    let user =  User::new(body.username, body.password);
    
    get_user_by_username(&user.get_username());

    let response = Json(json!({
        "message": "User authenticated"
    }));

    Ok(response)
}

