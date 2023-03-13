use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::models::user::User;


#[derive (Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub author: User,
    pub content: String,
    pub likes: u32,
}

impl Tweet {
    pub fn new(author: User, content: String) -> Tweet {
        Tweet {
            id: Uuid::new_v4().to_string(),
            author,
            content,
            likes: 0,
        }
    }
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    pub content: String,
    pub user_id: String,
}
