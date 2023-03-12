use rusqlite::{Connection, Result};

use crate::repository::get_connection;
use crate::models::tweet::{Tweet, TweetModel};
use crate::models::user::User;
use crate::repository::user::get_user_by_id;

struct TweetQueryResponse {
    id: String,
    content: String,
    author_id: String,
    author_username: String,
    author_password: String,
}

pub fn get_tweet(id: &String) -> Result<Tweet> {
    let connection = get_connection()?;
    
    let model = connection.query_row(
        "SELECT tweet.id, content, user.id, username, password FROM tweet JOIN user ON tweet.author = user.id WHERE id = ?1",
        &[&id],
        |row| {
            Ok(TweetQueryResponse {
                id: row.get(0)?,
                content: row.get(1)?,
                author_id: row.get(2)?,
                author_username: row.get(3)?,
                author_password: row.get(4)?,
            })
        },
    );

    Ok(Tweet {
        id: model?.id,
        content: model?.content,
        author: User {
            id: model?.author_id,
            username: model?.author_username,
            password: model?.author_password,
        },
        likes: vec![],
    })
}

pub fn get_tweet_by_id(id: &String) -> Result<Tweet> {
    let connection =  Connection::open("twitter.db")?;

    let model = connection.query_row(
        "SELECT id, content, author  FROM tweet WHERE id = ?1",
        &[&id],
        |row| {
            Ok(TweetModel {
                id: row.get(0)?,
                content: row.get(1)?,
                author_id: row.get(2)?,
            })
        },
    );

    let author = get_user_by_id(&model?.author_id)?;
    
    Ok(Tweet {
        id: model?.id,
        content: model?.content,
        author,
        likes: vec![]
    })
}
