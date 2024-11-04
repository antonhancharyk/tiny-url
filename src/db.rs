use rusqlite::{params, Connection};

pub fn init_db() -> Connection {
    let conn = Connection::open_in_memory().expect("Failed to initialize database.");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY,
            url TEXT NOT NULL,
            tiny_url TEXT NOT NULL UNIQUE
        )",
        [],
    )
    .expect("Failed to create table.");
    conn
}

pub fn insert_url(conn: &Connection, url: &str, tiny_url: &str) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO urls (url, tiny_url) VALUES (?1, ?2)",
        params![url, tiny_url],
    )
}
