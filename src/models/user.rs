use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds::serialize as to_ts;

#[derive (Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
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
pub struct UserModel {
    pub id: String, 
    pub username: String,
    pub password: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive (Debug, Serialize, Deserialize)]
pub struct AuthenticateUserRequest {
    pub username: String,
    pub password: String,
}

