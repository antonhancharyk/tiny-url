use rusqlite::Result;

mod db;

fn main() -> Result<()> {
    let _ = db::start().unwrap();

    Ok(())
}
