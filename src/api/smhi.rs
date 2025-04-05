use reqwest::Error;


pub async fn get_raw_json() -> Result<String, Error> {
    println!("Hej från api/smhi.test");
    let url = "https://opendata-download-metobs.smhi.se/api/version/latest/parameter/29/station/72420/period/latest-day/data.json";
    let respons = reqwest::get(url).await?.text().await?;
    Ok(respons)

}

