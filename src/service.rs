use crate::db;
use crate::models::NewUrl;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub fn create_url(form: NewUrl, db: Arc<Mutex<Connection>>) -> Result<String, String> {
    let conn = db.lock().map_err(|_| "Database lock error")?;
    db::insert_url(&conn, &form.url, &form.tiny_url)
        .map_err(|_| "Failed to insert URL".to_string())
        .map(|_| "Short URL created successfully".to_string())
}
