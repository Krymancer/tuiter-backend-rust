use uuid::Uuid;
use serde::{Serialize, Deserialize};


#[derive (Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            password
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

