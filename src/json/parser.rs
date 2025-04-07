// use serde::Deserialize;
use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct Observation {
    pub parameter: Parameter,
    pub station: Station,
    pub period: Period,
    pub value: Vec<Value>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub key: String,
    pub name: String, 
    summary: String,
    unit: String,
}

#[derive(Debug, Deserialize)]
pub struct Station {
    pub name: String,
    pub owner: String,
}

#[derive(Debug, Deserialize)]
pub struct Period {
    pub key: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    pub date: i64,
    #[serde(deserialize_with = "parse_tempvalue")]
    pub value: Option<f64>,
    pub quality: String,
}

fn parse_tempvalue<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
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
                        .map_err(serde::de::Error::custom)
                    }
                } 
                None => Ok(None)
            }
        }