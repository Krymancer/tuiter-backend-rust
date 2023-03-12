use rusqlite::{Connection, Result};

use crate::models::user::User;

pub fn get_user_by_id(id: &String) -> Result<User> {
    let connection =  Connection::open("twitter.db")?;

    connection.query_row(
        "SELECT id, username, password FROM user WHERE id = ?1",
        &[&id],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        },
    )
}

pub fn get_user_by_username(username: &String) -> Result<User> {
    let connection =  Connection::open("twitter.db")?;

    connection.query_row(
        "SELECT id, username, password FROM user WHERE username = ?1",
        &[&username],
        |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        },
    )
}


pub async fn insert_user(user: &User) -> Result<()> {
    let connection =  Connection::open("twitter.db")?;

    connection.execute(
        "INSERT INTO user (id, username, password) VALUES(?1, ?2, ?3)",
        &[&user.id, &user.username, &user.password],
    )?;

    Ok(())
}
