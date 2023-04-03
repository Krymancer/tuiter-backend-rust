use axum::{
    response::{Response, IntoResponse, Json},
    extract::{Path, State},
    routing::{post, get},
    http::StatusCode,
    Router
};

use serde_json::json;
use uuid::Uuid;

use crate::{
    models::tweet::{CreateTweetRequest, TweetQuery}, 
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
            COUNT(Likes.tweet) AS "likes!: i32"
            FROM Tweet 
            JOIN User ON Tweet.author = User.id
            LEFT JOIN Likes ON Tweet.id = Likes.tweet
            GROUP BY Tweet.id
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

pub async fn delete_tweet(
    auth_user: AuthUser,
    context: State<ApiContext>,
    Path(id) : Path<String>
) -> Response {
    let author_id = auth_user.user_id.to_string();

    let tweet = sqlx::query_scalar!(
        r#"
        SELECT
            Tweet.id AS "id!"
        FROM Tweet
        WHERE Tweet.id = $1 AND Tweet.author = $2
        "#,
        id, author_id
        )
        .fetch_one(&context.database)
        .await;
    
    let tweet = match tweet {
        Ok(tweet) => tweet,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({
            "message": "cannot find this tweet from this user in database"
        }))).into_response()
    };

    sqlx::query!(
        r#"
        DELETE FROM Tweet WHERE id = $1
        "#,
        id,
        )
        .execute(&context.database)
        .await
        .expect("failed to delete tweet");

    (StatusCode::OK, Json(json!({
        "message": "tweet deleted",
        "id": tweet
    }))).into_response()
} 
 

pub async fn get_tweet(
    context: State<ApiContext>,
    Path(id) : Path<String>
) -> Response {
    let tweet = sqlx::query_as!(
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
            COUNT(Likes.tweet) AS "likes!: i32"
            FROM Tweet 
            JOIN User ON Tweet.author = User.id
            LEFT JOIN Likes ON Tweet.id = Likes.tweet
            WHERE Tweet.id = $1
            GROUP BY Tweet.id
        "#,
        id
    )
    .fetch_one(&context.database)
    .await;
    
    let tweet = match tweet {
        Ok(tweet) => tweet,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({
            "message": "Cloud not find tweet with id provided"
        }))).into_response()
    };

    (StatusCode::OK, Json(json!({
        "tweet": tweet
    }))).into_response()
}
