use crate::api::smhi;
use crate::db::operation::insert_into_station;
use crate::api::smhi::{fetch_observation, Parametrar};
use rusqlite::Connection;
use tokio::sync::broadcast::error;


pub async fn import_station_data(conn: Connection, station_id: u32, parameter: &str) -> Result<(), Box<dyn std::error::Error>> {
    let observationer = fetch_observation(Parametrar::Temperatur, station_id).await?;

    Ok(())
}