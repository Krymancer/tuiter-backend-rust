use chrono::{DateTime, NaiveDateTime, Utc};
use anyhow::{Error, Context};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash};

pub fn deserialize_date_time(serialized_date: &String) -> Result<DateTime<Utc>, Error> {
    let timestamp = serialized_date.parse::<i64>().unwrap();
    let naive_datetime = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    if let Some(datetime) = naive_datetime {       
        let datetime_utc = DateTime::<Utc>::from_utc(datetime, Utc);
        Ok(datetime_utc)
    } else {  
        Err(Error::msg("Error parsing date"))
    }
}

pub async fn hash_password(password: String) -> Result<String, Error> {
    // See https://en.wikipedia.org/wiki/Argon2
    Ok(tokio::task::spawn_blocking(move || -> Result<String, Error> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_salt())
                .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
                .to_string(),
        )
    })
    .await
    .context("Error generating password hash")??)
}

pub async fn verify_password(password: String, password_hash: String) -> Result<(), Error> {
    Ok(tokio::task::spawn_blocking(move || -> Result<(), Error> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::msg("Unathorized"),
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into(),
            })
    })
    .await
    .context("panic in verifying password hash")??)
}
