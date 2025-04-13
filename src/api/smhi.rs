use reqwest::Error;
use crate::json::parser::Observation;

#[derive(Clone, Debug)]
pub enum Parametrar {
    Temperatur,
    Daggpunkt,
    Luftryck,
    Luftfuktighet,
    Molnmangd,
    Molnbas1,
    Vindhastighet,
    Vindriktning,
    Sikt,
}

impl Parametrar {
    pub fn to_id(&self) -> u32 {
        match self {
            Parametrar::Temperatur => 1,
            Parametrar::Daggpunkt => 39,
            Parametrar::Luftryck => 9,
            Parametrar::Luftfuktighet => 6,
            Parametrar::Molnmangd => 29,
            Parametrar::Molnbas1 => 28,
            Parametrar::Vindhastighet => 4,
            Parametrar::Vindriktning => 3,
            Parametrar::Sikt => 12,
        }
    }
}

pub fn build_url(parametrar: Parametrar, station: u32) -> String {
    format!(
        "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/{}/station/{}/period/latest-day/data.json",
        parametrar.to_id(),
        station
    )
}

pub async fn fetch_observation(url: &str) -> Result<Observation, Error> {
    let response = reqwest::get(url).await?;
    let obs: Observation = response.json().await?;
    Ok(obs)
}
