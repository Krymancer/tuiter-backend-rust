use sqlx::sqlite::SqlitePool;
use anyhow::{Error, Context};
use clap::Parser;
use twitter_backend_rust::config::Config;
use twitter_backend_rust::router::serve;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let config = Config::parse();

    let pool = SqlitePool::connect("./twitter.db")
        .await
        .context("Failed to connect to database")?;

    serve(config, pool).await?;
    
    Ok(())
    }
