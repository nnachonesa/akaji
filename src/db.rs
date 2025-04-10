use rusqlite::{Connection, Result};

pub fn init() -> Result<Connection> {
    let connec = Connection::open("sql.db")?;

    connec.execute(
        "CREATE TABLE IF NOT EXISTS movimientos (
                id INTEGER PRIMARY KEY,
                tipo TEXT NOT NULL,
                monto REAL NOT NULL,
                motivo TEXT NOT NULL,
                fecha TEXT NOT NULL,
                exclusive TEXT NOT NULL,
                categoria INTEGER)",
        [],
    )?;
    Ok(connec)
}
