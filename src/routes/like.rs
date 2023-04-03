use axum::{
    response::{Response, Json, IntoResponse},
    routing::post, Router, http::StatusCode,
    extract::{Path, State}
};

use serde_json::json;
use uuid::Uuid;

use crate::{router::ApiContext, extractor::AuthUser};

pub fn create_route() -> Router<ApiContext> {
    Router::new()
        .route("/like/:id", post(create_like).delete(delete_like))
}

pub async fn create_like(
    auth_user: AuthUser,
    context: State<ApiContext>,
    Path(id): Path<String>
) -> Response {
    let user_id = auth_user.user_id.to_string();
    let like_id = Uuid::new_v4().to_string();

    let like = sqlx::query!(
        r#"
            INSERT INTO Likes
            (id, tweet, user) VALUES ($1, $2, $3)
            RETURNING id, tweet, user
        "#,
        like_id, id, user_id
    )
    .fetch_one(&context.database)
    .await;

    let _ = match like {
        Ok(like) => like,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"message": "cloud not like the tweet"}))).into_response()
    };

    (StatusCode::OK, Json(json!({"message": "Tweet liked"}))).into_response()
}


pub async fn delete_like(
    auth_user: AuthUser,
    context: State<ApiContext>,
    Path(id): Path<String>
) -> Response {
    let user_id = auth_user.user_id.to_string();

    let like = sqlx::query!(
        r#"
            DELETE FROM Likes WHERE id = $1 AND user = $2
        "#,
        id, user_id
    )
    .fetch_optional(&context.database)
    .await;

    let _ = match like {
        Ok(like) => like,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({"message": "cloud not unlike the tweet"}))).into_response()
    };

    (StatusCode::OK, Json(json!({"message": "Tweet unliked"}))).into_response()
}
