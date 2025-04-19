use rusqlite::{Connection, Result};

pub fn create_station_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS station (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        latitude REAL NOT NULL,
        longitude REAL NOT NULL
    )",
    [],
)?;
Ok(())
}

pub fn create_observation_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS observations (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        station_id INTEGER NOT NULL,
        timestamp TEXT NOT NULL,
        parameter TEXT NOT NULL,
        value REAL NOT NULL,
        FOREIGN KEY(station_id) REFERENCES station(id),
        UNIQUE (station_id, timestamp, parameter)
    )",
[],
    )?;
Ok(())
}