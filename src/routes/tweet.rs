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
    models::tweet::{CreateTweetRequest, Tweet, TweetQuery}, 
    extractor::AuthUser, 
    router::ApiContext
};

pub fn create_route() -> Router<ApiContext> {
    Router::new()
        .route("/tweet", post(create_tweet).get(get_tweets))
        .route("/tweet/:id", get(get_tweet).delete(delete_tweet))
}

pub async fn create_tweet(
    auth_user: AuthUser,
    context: State<ApiContext>,
    Json(request): Json<CreateTweetRequest>
) -> Response {
    let user_id = auth_user.user_id.to_string();
    let uuid = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now().naive_utc().timestamp().to_string();

    let tweet = sqlx::query!(
        r#"
        INSERT INTO Tweet (id, content, author, created_at) values ($1, $2, $3, $4)
        RETURNING id, content, author, created_at
        "#,
        uuid,
        request.content,
        user_id,
        created_at
    )
    .fetch_one(&context.database)
    .await.unwrap();

    (StatusCode::OK, Json(json!({
        "message": "ok",
        "tweet": {
            "id": tweet.id,
            "content": tweet.content,
            "author": tweet.author,
            "created_at": tweet.created_at,
        }
    }))).into_response()
} 



pub async fn get_tweets(context: State<ApiContext>) -> Response {
    let tweets = sqlx::query_as!(
        TweetQuery,
        r#"
        SELECT
            Tweet.id AS "id!", 
            Tweet.content AS "content!",
            Tweet.created_at AS "created_at!",
            User.id AS "author_id!",
            User.username AS "author_username!", 
            User.hash AS "author_hash!",
            User.bio AS "author_bio!", 
            User.icon AS "author_icon!",
            User.created_at AS "author_created_at!",
            0 AS "likes!"
            FROM Tweet 
            JOIN User ON Tweet.author = User.id
        "#,
    )
    .fetch_all(&context.database)
    .await;
    
    let tweets = match tweets {
        Ok(tweets) => tweets,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "message": "internal server error"
        }))).into_response()
    };

    (StatusCode::OK, Json(json!({
        "message": "ok",
        "tweets": tweets
    }))).into_response()
}

pub async fn delete_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "DELETE", "id": id})))
} 
 

pub async fn get_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "GET", "id": id})))
}
