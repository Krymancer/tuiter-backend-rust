use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::serialize as to_ts;
use anyhow::Error;

use crate::utils::deserialize_date_time;


#[derive (Debug, Serialize, Deserialize, Default)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub hash: String,
    pub icon: String,
    pub bio: String,
    #[serde(serialize_with = "to_ts")]
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, password: String) -> Self { 
        Self {
            id: Uuid::new_v4(),
            username,
            hash: password,
            icon: "".to_string(),
            bio: "".to_string(),
            created_at: Utc::now(),
        }
    }
}

#[derive (Debug, Serialize, Deserialize)]
pub struct UserQuery {
    pub id: String,
    pub username: String,
    pub hash: String,
    pub icon: String,
    pub bio: String,
    pub created_at: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub icon: String,
    pub bio: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct AuthenticateUserRequest {
    pub username: String,
    pub password: String,
}

impl TryFrom<UserQuery> for User {
    type Error = Error; 
    fn try_from(value: UserQuery) -> Result<Self, Error> {
        Ok(User {
            id: Uuid::parse_str(&value.id)?,
            username: value.username,
            hash: value.hash,
            icon: value.icon,
            bio: value.bio,
            created_at: deserialize_date_time(&value.created_at)?,
        })
    }
}
