use chrono::Utc;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::models::user::User;
use crate::models::like::Like;


#[derive (Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub author: User,
    pub content: String,
    pub likes: Vec<Like>,
}

impl Tweet {
    fn new(author: User, content: String) -> Tweet {
        Tweet {
            id: Uuid::new_v4().to_string(),
            author,
            content,
            likes: Vec::new(),
        }
    }
}

#[derive (Debug, Serialize, Deserialize)]
pub struct TweetModel {
    pub id: String,
    pub content: String,
    pub author_id: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    content: String,
    user_id: String,
}
