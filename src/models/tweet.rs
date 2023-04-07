use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use anyhow::Error;

use crate::{models::user::{User, UserQuery}, utils::deserialize_date_time};

#[derive (Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub id: Uuid,
    pub author: User,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub likes: i32,
}

impl Tweet {
    pub fn new(author: User, content: String) -> Tweet {
        Tweet {
            id: Uuid::new_v4(),
            author,
            content,
            likes: 0,
            created_at: Utc::now(),
        }
    }
}

impl TryFrom<TweetQuery> for Tweet {
    type Error = Error;

    fn try_from(value: TweetQuery) -> Result<Self, Self::Error> {
        let author_query = UserQuery {
            id: value.author_id,
            username: value.author_username,
            hash: value.author_hash,
            bio: value.author_bio,
            icon: value.author_icon,
            created_at: value.author_created_at,
        };


        let author = User::try_from(author_query)?;

        Ok(Tweet {
            id: Uuid::parse_str(&value.id)?,
            author,
            content: value.content,
            likes: value.likes,
            created_at: deserialize_date_time(&value.created_at)?,
        })
    }
}

pub fn from_vec(values: Vec<TweetQuery>) -> Result<Vec<Tweet>, Error> {
    values.into_iter().map(Tweet::try_from).collect()
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CreateTweetRequest {
    pub content: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct TweetQuery {
    pub id: String,
    pub content: String,
    pub created_at: String,
    pub author_id: String,
    pub author_username: String,
    pub author_hash: String,
    pub author_bio: String,
    pub author_icon: String,
    pub author_created_at: String,
    pub likes: i32,
}
