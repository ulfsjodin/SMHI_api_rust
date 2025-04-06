mod api;
// use std::os::linux::raw::stat;

// use api::smhi::get_raw_json;
use api::smhi::{fetch_json, build_url, Parametrar};

#[tokio::main]
async fn main() {
    let station = 72420;

    let url_temp = build_url(Parametrar::Temperatur, station);
    let url_tryck = build_url(Parametrar::Luftryck, station);

    let jsontemperatur = fetch_json(&url_temp).await.unwrap();
    let jsontryck = fetch_json(&url_tryck).await.unwrap();

    println!("hela json med temperaturer: {}", jsontemperatur);
    println!("--------------------------------------------------------");
    println!("hela json med lufttryck: {}", jsontryck);

    // match get_raw_json().await {
    //     Ok(json) => println!("Svar från SMHI: {:?}", json),
    //     Err(e) => eprintln!("Inget svar fån SMHI {}", e),
    // }
}
