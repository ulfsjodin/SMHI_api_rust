mod api;
mod json;

use api::smhi::{fetch_observation, build_url, Parametrar};
use json::parser;

#[tokio::main]
async fn main() {
    // Välj station
    // let station_id = 72420; // Landvetter
    // let station_id = 105260; // Borlänge flygplats
    // let station_id = 62410;  // Halmstad flygplats
    // let station_id = 155970; // Hemavans flygplats
    // let station_id = 74460; // Jönköping Axamo flygplats
    // let station_id = 53300; // Malmö Sturup flygplats
    // let station_id = 97400; // Stockholm Arlanda flygplats
    let station_id = 127310; // Sundsvall Timrå flygplats
    
    // Välj parameter
    // let parameter = Parametrar::Molnmangd; 
    // let parameter = Parametrar::Temperatur; 
    // let parameter = Parametrar::Luftryck; 
    // let parameter = Parametrar::Daggpunkt; 
    // let parameter = Parametrar::Molnbas1; 
    // let parameter = Parametrar::Vindhastighet; 
    let parameter = Parametrar::Vindriktning; 
    
    let url = build_url(parameter, station_id);

    match fetch_observation(&url).await {
        Ok(obs) => {
            println!("Observation för station {}:", station_id);
            println!("OBS! : {:#?}", &obs);
        }
        Err(e) => eprintln!("Fel vid hämtning: {}", e),
    }
}
