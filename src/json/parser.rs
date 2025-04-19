use serde::Deserialize;
use crate::json::tempvalue_parser::parse_tempvalue;
use crate::json::timestamp_converter::deserialize_millis_to_cest;

#[derive(Debug, Deserialize)]
pub struct Observation {
    pub parameter: Parameter,
    pub station: Station,
    pub period: Period,
    pub position: Vec<Position>,
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
    pub ownerCategory: String,
    pub measuringStations: String,
    pub height: f64,
}

#[derive(Debug, Deserialize)]
pub struct Period {
    pub key: String,
    #[serde(deserialize_with = "deserialize_millis_to_cest")]
    pub from: String,
    #[serde(deserialize_with = "deserialize_millis_to_cest")]
    pub to: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
pub struct Position {
    #[serde(deserialize_with = "deserialize_millis_to_cest")]
    pub from: String,
    #[serde(deserialize_with = "deserialize_millis_to_cest")]
    pub to: String,
    pub height: f64,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Deserialize)]
pub struct Value {
    #[serde(deserialize_with = "deserialize_millis_to_cest")]
    pub date: String,
    #[serde(deserialize_with = "parse_tempvalue")]
    pub value: Option<f64>,
    pub quality: String,
}