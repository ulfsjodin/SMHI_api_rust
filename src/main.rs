mod db;
mod api;
mod json;
mod importer;

use omstart_smhi::parser::Position;
// use omstart_smhi::schema;
use rusqlite::Connection;
use api::smhi::{fetch_observation, Parametrar};
use db::{connection::open_connection, operation};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    // Välj station
    // let station_id = 72420; // Landvetter
    // let station_id = 105260; // Borlänge flygplats
    // let station_id = 62410;  // Halmstad flygplats
    // let station_id = 155970; // Hemavans flygplats
    // let station_id = 74460; // Jönköping Axamo flygplats
    // let station_id = 53300; // Malmö Sturup flygplats
    // let station_id = 97400; // Stockholm Arlanda flygplats
    // let station_id = 127310; // Sundsvall Timrå flygplats
    
    // Välj parameter
    // let parameter = Parametrar::Molnmangd; 
    // let parameter = Parametrar::Temperatur; 
    // let parameter = Parametrar::Luftryck; 
    // let parameter = Parametrar::Daggpunkt; 
    // let parameter = Parametrar::Molnbas1; 
    // let parameter = Parametrar::Vindhastighet; 
    // let parameter = Parametrar::Vindriktning; 
    
    let station_ids = vec![53300];
    let parameters = vec![Parametrar::Temperatur, Parametrar::Luftryck, Parametrar::Vindhastighet, Parametrar::Vindriktning];
    
    //  för testning
    // let stationen= 53300;
    // let parameters = vec![Parametrar::Temperatur];
    // let getit = fetch_observation(Parametrar::Temperatur, stationen).await;
    // println!("<<<test>>>: {:#?}", getit);
    
    for vaderobs in station_ids {
        for params in &parameters {
            // let url = build_url(params.clone(), vaderobs);
            
            match fetch_observation(params.clone(), vaderobs).await {
                Ok(obs) => {
                    println!("_______________________");
                    println!("Data från station/er: {},  {:?}", vaderobs, params);
                    println!("-----------------------");
                    // println!("Observationen >: {:#?}",  obs);
                }
                Err(e) => {
                    eprintln!("❌ Fel vid hämtning från station {} med parameter {:?}:\n{}", vaderobs, params, e);
                }
            }
        }
    }

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


