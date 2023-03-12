use rusqlite::{Connection, Result};

use crate::models::like::{Like, LikeModel};
use crate::repository::user::get_user_by_id;
use crate::repository::tweet::get_tweet_by_id;

pub async fn get_likes_from_tweet_id(tweet_id: String) -> Result<Vec<Like>> {
    let connection = Connection::open("tweet.db")?;

    let tweet = get_tweet_by_id(&tweet_id.clone())?;

    let mut statement = connection.prepare("SELECT tweet FROM like WHERE tweet = :id")?;
    let mut rows = statement.query_map(
        &[(":id", &tweet_id)],
        |row| {
            Ok(LikeModel {
                id: row.get(0)?,
                tweet_id: row.get(1)?,
                author_id: row.get(2)?,
            })
        }
    )?;

    let mut likes = Vec::new();

    for row in rows {
        let user = get_user_by_id(&row?.author_id)?;

        let like = Like {
            id: row?.id,
            tweet,
            author: user,
        };

        likes.push(like);
    }

    Ok(likes)
}
