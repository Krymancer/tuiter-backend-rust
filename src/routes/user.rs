use axum::{
    response::{Response, IntoResponse, Json},
    routing::post,
    http::StatusCode,
    Router,
    Extension
};
use anyhow::{Error, Context};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};
use serde_json::{json, Value};
use uuid::Uuid;
use chrono::{Utc, TimeZone};

use sqlx::error::DatabaseError;

use crate::router::ApiContext;
use crate::models::user::{CreateUserRequest, AuthenticateUserRequest};

pub fn create_route() -> Router {
    Router::new()
        .route("/user", post(create_user))
        .route("/auth", post(authenticate_user))
}

async fn create_user(
    context: Extension<ApiContext>,
    Json(request): Json<CreateUserRequest>
) -> Response {

    let created_at = chrono::Utc::now().naive_utc().timestamp().to_string();
    let hash_password = hash_password(request.password).await;

    let hash_password = match hash_password {
        Ok(hash_password) => hash_password,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Unable to hash password"}))).into_response()
    };

    let uuid = Uuid::new_v4().to_string();

    let user_id = sqlx::query_scalar!(
        r#"INSERT INTO User
        (id, username, hash, icon, bio, created_at)
        values
        ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
        uuid,
        request.username,
        hash_password,
        request.icon,
        request.bio,
        created_at
    )
    .fetch_one(&context.database)
    .await;
    
    let user_id = match user_id {
        Ok(id) => id,
        Err(error) => match error {
            // Here we need to check by code 2067 (Unique Constraint Code) because the sqlite
            // driver does not implement the constraint() method on the error object
            // We also cant check if is a different constraint, but as in the user table we only
            // have a constraint in user, we should be fine
            sqlx::Error::Database(e) if "2067" == e.code().as_ref().expect("Error must have a code").to_string() => return (StatusCode::BAD_REQUEST, Json(json!({"message": "Username Taken"}))).into_response(),
            _ => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Unable to create user"}))).into_response()
        } 
    };

    let user_id = match user_id {
        Some(id) => id,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Something went wrong"}))).into_response()
    };

    (StatusCode::OK, Json(json!({
        "message": "User Created!",
        "user": {
            "id": user_id
        }
    }))).into_response()
}

async fn authenticate_user(
    context: Extension<ApiContext>,
    Json(request) : Json<AuthenticateUserRequest>
) -> Response {

    let user = sqlx::query!(
        r#"SELECT
        id, username, hash, icon, bio, created_at
        FROM User WHERE username = $1 LIMIT 1
        "#,
        request.username
        )
        .fetch_optional(&context.database)
        .await;

    let user = match user {
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"message": "User not found"}))).into_response(),
        Ok(user) => match user {
            Some(user) => user,
            None => return (StatusCode::NOT_FOUND, Json(json!({"message": "User not found"}))).into_response()
        }
    };

    let uuid = match Uuid::parse_str(&user.id.as_str()) {
        Ok(id) => id,
        Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"message": "User not found"}))).into_response()
    };

    let timestamp = match user.created_at.parse::<i64>(){
        Ok(timestamp) => timestamp,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response()
    }
    ;
    let created_at = match Utc.timestamp_opt(timestamp, 0) {
        chrono::LocalResult::None => return (StatusCode::INTERNAL_SERVER_ERROR, ()).into_response(),
        chrono::LocalResult::Single(created_at) => created_at,
        chrono::LocalResult::Ambiguous(created_at, _) => created_at
    };

    let verify = verify_password(request.password, user.hash).await;

    let verify = match verify {
        Ok(_) => return (StatusCode::OK, Json(json!({"message": "User authenticated"}))).into_response(),
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"message": "Invalid password"}))).into_response()
    };
}

async fn hash_password(password: String) -> Result<String, Error> {
    // See https://en.wikipedia.org/wiki/Argon2
    Ok(tokio::task::spawn_blocking(move || -> Result<String, Error> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_salt())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("Error generating password hash")??)
}

async fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
    Ok(tokio::task::spawn_blocking(move || -> Result<(), Error> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::msg("Unathorized"),
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}
