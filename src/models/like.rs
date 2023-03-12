use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::Utc;

use crate::models::user::User;
use crate::models::tweet::Tweet;

#[derive (Debug, Serialize, Deserialize)]
pub struct Like {
    pub id: String,
    pub tweet: Tweet,
    pub author: User,
}

impl Like {
   pub fn new(tweet: Tweet, author: User) -> Like {
       Like {
           id: Uuid::new_v4().to_string(),
           tweet,
           author,
       }
   } 
}

#[derive (Debug, Serialize, Deserialize)]
pub struct LikeModel {
    pub id: String,
    pub tweet_id: String,
    pub author_id: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CreateLikeRequest {
    tweet_id: String,
    user_id: String,
}
