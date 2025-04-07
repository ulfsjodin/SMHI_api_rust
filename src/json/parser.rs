use serde::Deserialize;

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
    pub value: String,
    pub quality: String,
}