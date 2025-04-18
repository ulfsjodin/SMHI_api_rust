use rusqlite::{Connection, Result, params};

pub fn insert_into_station(conn: &Connection, id: i32, name: &str, lat: f64, long: f64 ) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO station (id, name, latitude, longitude) 
        VALUES (?1, ?2, ?3)", params![id, name, lat, long],
    )?;
Ok(())
}