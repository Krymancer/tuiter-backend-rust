use axum::{
    response::{Response, IntoResponse, Json},
    extract::Path,
    routing::{post, delete, get},
    http::StatusCode,
    Router
};

use serde_json::{json, Value};

use crate::models::tweet::{CreateTweetRequest, Tweet};

use crate::repository::user::get_user_by_id;
use crate::repository::tweet::{insert_tweet, get_tweet_by_id};

pub fn create_route() -> Router {
    Router::new()
        .route("/tweet", post(create_tweet))
        .route("/tweet/:id", delete(delete_tweet))
        .route("/tweet/:id", get(get_tweet))
}


pub async fn create_tweet(Json(body) :  Json<CreateTweetRequest> ) -> Response {
    let author = get_user_by_id(&body.user_id);

    if author.is_err() {
        return (StatusCode::NOT_FOUND, Json(json!({"message": "Username not found!"}))).into_response();
    }

    let author = author.unwrap(); 

    let tweet = Tweet::new(author, body.content);
    
    let result = insert_tweet(tweet);

    println!("{:?}", result);
    
    if result.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Internal server error!"}))).into_response();
    }

    (StatusCode::OK, Json(json!({ "message": "tweet create created"}))).into_response() 
} 
 

pub async fn delete_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "DELETE", "id": id})))
} 
 

pub async fn get_tweet(Path(id) : Path<String>) -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "path": "/tweet/:id", "method": "GET", "id": id})))
}
