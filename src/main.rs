mod api;
mod json;

use api::smhi::{fetch_observation, build_url, Parametrar};
// use json::parser;

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
    
    let station_ids = vec![72420];
    let parameters = vec![Parametrar::Temperatur, Parametrar::Luftryck, Parametrar::Vindhastighet, Parametrar::Vindriktning];
    for vaderobs in station_ids {
        for params in &parameters {
            let url = build_url(params.clone(), vaderobs);
            
            match fetch_observation(&url).await {
                Ok(obs) => {
                    println!("_______________________");
                    println!("Data från station/er: {},  {:?}", vaderobs, params);
                    println!("-----------------------");
                    println!("Observationen >: {:#?}",  obs);
                }
                Err(e) => {
                    eprintln!("❌ Fel vid hämtning från station {} med parameter {:?}:\n{}", vaderobs, params, e);
                }
            }
        }
    }

    Ok(())
            
    }

