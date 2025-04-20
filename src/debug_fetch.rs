use crate::api::smhi::{fetch_observation, Parametrar};
// src/debug_fetch.rs

pub async fn test_fetch() {

    let station_ids = vec![53300];
    let parameters = vec![
        Parametrar::Temperatur,
        Parametrar::Luftryck,
        Parametrar::Vindhastighet,
        Parametrar::Vindriktning,
    ];

    for station_id in station_ids {
        for param in &parameters {
            match fetch_observation(param.clone(), station_id).await {
                Ok(obs) => {
                    println!("_______________________");
                    println!("Data från station: {},  {:?}", station_id, param);
                    println!("-----------------------");
                    println!("{:#?}", obs);
                }
                Err(e) => {
                    eprintln!("❌ Fel vid hämtning från station {} med parameter {:?}:\n{}", station_id, param, e);
                }
            }
        }
    }
}
