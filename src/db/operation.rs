use rusqlite::{Connection, Result, params};

pub fn insert_into_station(conn: &Connection, id: i32, name: &str, lat: f64, long: f64 ) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO station (id, name, latitude, longitude) 
        VALUES (?1, ?2, ?3, ?4)", params![id, name, lat, long],
    )?;
Ok(())
}

pub fn insert_into_observations(
    conn: &Connection, 
    station_id: i32, 
    timestamp: &str, 
    parameter: &str, 
    value: f64) -> Result<()> {
        conn.execute(
            "INSERT OR IGNORE INTO observations (station_id, timestamp, parameter, value)
            VALUES (?1, ?2, ?3, ?4)", params![station_id,timestamp, parameter, value],
        )?;
        Ok(())
    } 