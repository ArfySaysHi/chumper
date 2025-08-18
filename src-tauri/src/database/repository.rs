use crate::error::Result;
use crate::shared::get_ext_from;
use rusqlite::Connection;
use std::fs::{remove_file, File};
use std::io::Read;
use std::path::Path;

fn run_migration(connection: &Connection, path: String) -> Result<()> {
    let mut f = File::open(&path)?;
    let mut schema = String::new();
    f.read_to_string(&mut schema)?;

    println!("Running migration for: {:?}", &path);
    connection.execute_batch(&schema)?;
    println!("Success!");

    Ok(())
}

pub fn init_database(connection: &Connection) -> Result<()> {
    let path = Path::new("./chumper.db3");
    if path.exists() {
        remove_file(path)?;
    }
    let paths = get_ext_from("./db".to_owned(), ".sql".to_owned());

    for path in paths {
        run_migration(&connection, path)?;
    }

    Ok(())
}
