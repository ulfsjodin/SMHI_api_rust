mod db;
mod api;
mod json;
mod importer;
mod debug_fetch;

use rusqlite::Connection;
use api::smhi::{fetch_observation, Parametrar};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // för testning:
    // let testar = debug_fetch::test_fetch().await;
    // println!("TESTAR: {:?}", testar);

    let conn = Connection::open("ulf.db")?;
    conn.execute("PRAGMA foreign_keys = ON", [])?;
    db::schema::create_station_table(&conn)?;
    db::schema::create_observation_table(&conn)?;

    let station_ids = vec![105260];
    let parameters = vec![
        Parametrar::Temperatur,
        Parametrar::Luftfuktighet,
        Parametrar::Vindhastighet,
        Parametrar::Vindriktning,
    ];

    for station_id in station_ids {
        for params in &parameters {
            match fetch_observation(params.clone(), station_id).await {
                Ok(obs) => {
                    println!("Data från station {} med data {:#?}", station_id, params);

                    let station = &obs.station;
                    let id = station_id as i32;
                    let name = &station.name;

                    if let Some(pos) = obs.position.first() {
                        let lat = pos.latitude;
                        let long = pos.longitude;

                        if let Err(e) = db::operation::insert_into_station(&conn, id, name, lat, long) {
                            eprintln!("Något blev fel i main: loopen för inmatning: {}", e);
                        }

                        for v in &obs.value {
                            let timestamp = &v.date;
                            if let Some(val) = v.value {
                                // parameternamn som sträng
                                let param_name = params.to_string();

                            // mata in i tabellen härifrån
                            if let Err(e) = db::operation::insert_into_observations(
                                &conn, 
                                id, 
                                timestamp, 
                                &param_name, 
                                val,
                            ) {
                                eprintln!("Fel vid inmatning i observations {}", e);
                            }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Fel vid hämtning från station {} (SMHI) {}", station_id, e);
                }
            }
        }
    }
    
    Ok(())
            
    }


