pub mod user;
pub mod tweet;
pub mod like;

use rusqlite::Connection;
use anyhow::Error;

pub fn get_connection() -> Result<Connection, Error> {
  Ok(Connection::open("tweet.db")?)
}
