use rusqlite::Result;

mod db;
mod server;

// fn main() -> Result<()> {
fn main() {
    // let _ = db::start().unwrap();

    // Ok(());

    server::run();
}
