use reqwest::Error;
use crate::json::parser::Observation;
use std::fmt;

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

impl fmt::Display for Parametrar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Parametrar::Temperatur => "Temperatur",
            Parametrar::Daggpunkt => "Daggpunkt",
            Parametrar::Luftryck => "Lufttryck",
            Parametrar::Luftfuktighet => "Luftfuktighet",
            Parametrar::Molnmangd => "Molnmängd",
            Parametrar::Molnbas1 => "Molnbas1",
            Parametrar::Vindhastighet => "Vindhastighet",
            Parametrar::Vindriktning => "Vindriktning",
            Parametrar::Sikt => "Sikt",
        };
        write!(f, "{}", s)
    }
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
/// Bygger en URL för SMHI:s API givet en parameter och en stationskod.
///
/// # Exempel
///
/// ```
/// use omstart_smhi::api::smhi::{build_url, Parametrar};
///
/// let url = build_url(Parametrar::Temperatur, 12345);
/// assert_eq!(
///     url,
///     "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/1/station/12345/period/latest-day/data.json"
/// );
/// ```
pub fn build_url(parametrar: Parametrar, station: u32) -> String {
    format!(
        "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/{}/station/{}/period/latest-day/data.json",
        parametrar.to_id(),
        station
    )
}

pub async fn fetch_observation(param: Parametrar, station_id: u32) -> Result<Observation, Error> {
    let url = build_url(param, station_id);
    let response = reqwest::get(url).await?;
    let obs: Observation = response.json().await?;
    Ok(obs)
}
