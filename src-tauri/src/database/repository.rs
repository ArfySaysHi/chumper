use crate::error::Result;
use crate::shared::get_ext_from;
use rusqlite::Connection;
use std::fs::File;
use std::io::Read;

fn run_migration(connection: &Connection, path: String) -> Result<()> {
    log::info!("Running migration for {}", &path);
    let mut f = File::open(&path)?;
    let mut schema = String::new();
    f.read_to_string(&mut schema)?;

    connection.execute_batch(&schema)?;

    Ok(())
}

pub fn init_database(connection: &Connection) -> Result<()> {
    log::info!("Running init_database...");
    let paths = get_ext_from("./db".to_owned(), ".sql".to_owned());

    for path in paths {
        run_migration(&connection, path)?;
    }

    Ok(())
}
