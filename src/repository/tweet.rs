use rusqlite::{Connection, Result, Error};

use crate::repository::get_connection;
use crate::models::tweet::Tweet;
use crate::models::user::User;

struct TweetQueryResponse {
    id: String,
    content: String,
    author_id: String,
    author_username: String,
    author_password: String,
    likes: u32,
}

pub fn get_tweet(id: &String) -> Result<Tweet> {
    let connection = get_connection()?;
    
    let model = connection.query_row(
        "SELECT tweet.id, content, user.id, username, password, COUNT(like.id) FROM tweet JOIN user ON tweet.author = user.id JOIN like ON like.tweet = tweet.id WHERE tweet.id = ?1",
        &[&id],
        |row| {
            Ok(TweetQueryResponse {
                id: row.get(0)?,
                content: row.get(1)?,
                author_id: row.get(2)?,
                author_username: row.get(3)?,
                author_password: row.get(4)?,
                likes: row.get(5)?,
            })
        },
    );

    if let Err(e) = model {
        return Err(e);
    }

    let tweet_model = model.unwrap();


    Ok(Tweet {
        id: tweet_model.id,
        content: tweet_model.content,
        author: User {
            id: tweet_model.author_id,
            username: tweet_model.author_username,
            password: tweet_model.author_password,
        },
        likes: tweet_model.likes,
    })
}

pub fn get_tweet_by_id(id: &String) -> Result<Tweet> {
    let connection =  Connection::open("twitter.db")?;

    let model  = connection.query_row(
        "SELECT tweet.id, content, user.id, username, password, COUNT(like.id) FROM tweet JOIN user ON user.id = tweet.author JOIN like ON like.tweet = tweet.id WHERE tweet.id = ?1",
        &[&id],
        |row| Ok(TweetQueryResponse { 
                id: row.get(0)?,
                content: row.get(1)?,
                author_id: row.get(2)?,
                author_username: row.get(3)?,
                author_password: row.get(4)?,
                likes: row.get(5)?,
            })
    );

    if let Err(e) = model {
        return Err(e);
    }

    let tweet_model = model.unwrap();
 
    Ok(Tweet {
        id: tweet_model.id,
        content: tweet_model.content,
        author: User {
            id: tweet_model.author_id,
            username: tweet_model.author_username,
            password: tweet_model.author_password,
        },
        likes: tweet_model.likes,
    })
}

pub fn insert_tweet(tweet: Tweet) -> Result<usize, Error> {
    let connection =  Connection::open("twitter.db")?;

    connection.execute(
        "INSERT INTO tweet (id, content, author) values (?1, ?2, ?3)",
        (tweet.id, tweet.content, tweet.author.id)
    )
}
