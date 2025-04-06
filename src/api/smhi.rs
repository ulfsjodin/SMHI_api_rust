use reqwest::Error;

pub enum Parametrar {
    Temperatur,
    Luftryck,
    Molnmangd,
}

impl Parametrar {
    pub fn to_id(&self) -> u32 {
        match self {
            Parametrar::Temperatur => 1,
            Parametrar::Luftryck => 9,
            Parametrar::Molnmangd => 29,
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

pub async fn fetch_json(url: &str) -> Result<String, Error> {
    let respons = reqwest::get(url).await?.text().await?;
    Ok(respons)
}

// pub async fn get_raw_json() -> Result<(String, String), Error> {
//     println!("Hej från api/smhi.test");
//     let molnmangd = "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/29/station/72420/period/latest-day/data.json";
//     let respons = reqwest::get(molnmangd).await?.text().await?;
//     let lufttryck = "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/9/station/72420/period/latest-day/data.json";
//     let respons2 = reqwest::get(lufttryck).await?.text().await?;
//     Ok((respons, respons2))

// }

