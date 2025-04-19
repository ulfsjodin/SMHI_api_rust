use rusqlite::{params, Connection, Result};

pub fn open_connection(databasnamn: &str) -> Result<Connection> {
    // Här anges databasens filnamn om den redan inte är skapad.
    let conn = Connection::open(databasnamn)?;
    Ok(conn)
}