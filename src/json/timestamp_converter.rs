use chrono::{DateTime, Utc};
use chrono_tz::Europe::Stockholm;
use serde::{self, Deserialize, Deserializer};

pub fn millis_to_cest( millis: i64) -> Option<String> {
    DateTime::<Utc>::from_timestamp_millis(millis).map(|dt_utc| {
        let dt_cest = dt_utc.with_timezone(&Stockholm);
        dt_cest.format("%y-%m-%d %H:%M:%S").to_string()
    })
}

pub fn deserialize_millis_to_cest<'de, D>(deserializer: D) -> Result<String, D::Error>
    where 
        D: Deserializer<'de>,
        {
            let millis: i64 = Deserialize::deserialize(deserializer)?;
            millis_to_cest(millis).ok_or_else(|| serde::de::Error::custom("Ogiltig tidsstaämpel")) 
        }