use axum::{
    response::{Response, IntoResponse, Json},
    routing::post,
    http::StatusCode,
    Router
};
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

    let result = insert_user(&user).await;

    if result.is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let response = Json(json!({
        "user": user
    }));

    Ok(response)
}

async fn authenticate_user(Json(body) : Json<AuthenticateUserRequest>) -> Response {
    
    let user = User::new(body.username, body.password);

    let result = get_user_by_username(&user.username);

    if result.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"message": "User not found"}))).into_response();
    }

    if result.is_ok() {
        let user_db = result.unwrap();

        if user_db.password != user.password {
            return (StatusCode::UNAUTHORIZED, Json(json!({"message": "Username or password wrong"}))).into_response();
        }
    }

    let response = Json(json!({
        "message": "User authenticated"
    }));

    (StatusCode::OK, response).into_response()
}

