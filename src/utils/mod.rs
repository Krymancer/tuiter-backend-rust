use chrono::{DateTime, NaiveDateTime, Utc};
use anyhow::Error;


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
