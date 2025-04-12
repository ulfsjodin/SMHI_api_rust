use serde::Deserialize;
use serde::de::{self, Deserializer};
use std::str::FromStr;

pub fn parse_tempvalue<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(text) => {
            if text.trim().is_empty() || text == "NaN" {
                Ok(None)
            } else {
                f64::from_str(&text)
                    .map(Some)
                    .map_err(de::Error::custom)
            }
        }
        None => Ok(None),
    }
}
