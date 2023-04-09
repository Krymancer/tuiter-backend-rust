use axum::{
    response::{Response, IntoResponse, Json},
    extract::State,
    routing::post,
    http::StatusCode,
    Router
};
use serde_json::json;
use uuid::Uuid;
use crate::router::ApiContext;
use crate::utils::{hash_password, verify_password};
use crate::models::user::{CreateUserRequest, AuthenticateUserRequest, User, UserQuery};
use crate::extractor::AuthUser;

pub fn create_route() -> Router<ApiContext> {
    Router::new()
        .route("/user", post(create_user))
        .route("/auth", post(authenticate_user))
}

async fn create_user(
    context: State<ApiContext>,
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
            // driver does not populate the constraint() method on the error object
            // We also can't check if is an specific constraint, but as in the user table we only
            // have a constraint in user, we should be fine
            sqlx::Error::Database(e) 
                if "2067" == e.code().as_ref().expect("Error must have a code").to_string() => 
                    return (StatusCode::BAD_REQUEST, Json(json!({"message": "Username Taken"}))).into_response(),
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
    context: State<ApiContext>,
    Json(request) : Json<AuthenticateUserRequest>
) -> Response {
    let user = sqlx::query_as!(
        UserQuery,
        r#"SELECT
        id as "id!",
        username as "username!", 
        hash as "hash!",
        icon as "icon!",
        bio as "bio!",
        created_at  as "created_at!"
        FROM User WHERE username = $1 LIMIT 1
        "#,
        request.username
        )
        .fetch_optional(&context.database)
        .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Something went wrong"}))).into_response(),
    };

    let user = match user {
        Some(user) => user,
        None => return (StatusCode::NOT_FOUND, Json(json!({"message": "User not found"}))).into_response()
    };

    let user = User::try_from(user);

    let user = match user {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Something went wrong"}))).into_response()
    };

    let verify = verify_password(request.password, user.hash.clone()).await;

    let _verify = match verify {
        Ok(_) => return (StatusCode::OK, Json(json!({
            "message": "User authenticated", 
            "user": user,
            "token": AuthUser {
                user_id: user.id,
            }.to_jwt(&context)
        }))).into_response(),
        Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({"message": "Invalid password"}))).into_response()
    };
}


