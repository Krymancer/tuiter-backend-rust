use rusqlite::Connection;
use anyhow::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDateTime};

use crate::models::user::User;

use super::get_connection;


struct UserQuery {
    id: String,
    username: String,
    hash: String,
    icon: String,
    bio: String,
    created_at: String,
}


fn deserialize_date_time(serialized_date: &String) -> Result<DateTime<Utc>, Error> {
    let timestamp = serialized_date.parse::<i64>().unwrap();
    let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    if let Some(datetime) = naive_datetime {       
        let datetime_utc = DateTime::<Utc>::from_utc(datetime, Utc);
        Ok(datetime_utc)
    } else {  
        Err(Error::msg("Error parsing date"))
    }
}

fn user_query_to_user(user_query: UserQuery) -> Result<User, Error> {
    let created_at = DateTime::parse_from_rfc3339(&user_query.created_at);
    let uuid = Uuid::parse_str(&user_query.id);
    if let (Ok(created_at), Ok(uuid)) = (
        deserialize_date_time(&user_query.created_at),
        Uuid::parse_str(&user_query.id)) {
        Ok(User {
            id: uuid,
            username: user_query.username,
            hash: user_query.hash,
            icon: user_query.icon,
            bio: user_query.bio,
            created_at
        })
    } else {
        Err(Error::msg("Error parsing date or uuid"))
    }
}

pub fn get_user_by_uuid(id: &Uuid) -> Result<User, Error> {
    let connection = get_connection()?;

    let user_query = connection.query_row(
        "SELET id, username, hash, icon, bio, created_at FROM User WHERE id = ?1",
        &[&id.to_string()],
        |row| {
            Ok(UserQuery {
                id: row.get(0)?,
                username: row.get(1)?,
                hash: row.get(2)?,
                icon: row.get(3)?,
                bio: row.get(4)?,
                created_at: row.get(5)?,
            })
        });

    if let Ok(user_query) = user_query {
        user_query_to_user(user_query)
    } else {
        Err(Error::msg("Error in query"))
    }

}

pub fn get_user_by_username(username: &String) -> Result<User, Error> {
    let connection =  get_connection()?;

    let user_query = connection.query_row(
        "SELECT id, username, hash, icon, bio, created_at FROM User WHERE username = ?1",
        &[&username],
        |row| {
            Ok(UserQuery {
                id: row.get(0)?,
                username: row.get(1)?,
                hash: row.get(2)?,
                icon: row.get(3)?,
                bio: row.get(4)?,
                created_at: row.get(5)?,
            })
        },
    );
    
    if let Ok(user_query) = user_query {
        user_query_to_user(user_query)
    } else {
        Err(Error::msg("Error in query"))
    }
}

pub async fn insert_user(user: &User) -> Result<(), Error> {
    let connection =  Connection::open("twitter.db")?;

    connection.execute(
        "INSERT INTO User (id, username, hash, icon, bio, created_at) VALUES(?1, ?2, ?3, ?4, ?5, ?6)",
        &[
            &user.id.to_string(),
            &user.username,
            &user.hash,
            &user.icon,
            &user.bio,
            &user.created_at.timestamp().to_string()
        ]
    )?;

    Ok(())
}


