pub mod user;
pub mod tweet;
pub mod like;

use rusqlite::{Connection, Result};

pub fn get_connection() -> Result<Connection> {
  Ok(Connection::open("tweet.db")?)
}
