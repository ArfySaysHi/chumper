use crate::utils::error_handling::CommandResult;
use crate::utils::file_handling::get_ext_from;
use rusqlite::{Connection, Result};
use std::fs::{remove_file, File};
use std::io::Read;
use std::path::Path;

pub struct DatabaseController {
    pub db_path: String,
}

impl DatabaseController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db_path: "chumper.db3".to_string(),
        })
    }

    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    fn run_migration(&self, conn: &Connection, path: String) -> CommandResult<()> {
        let mut f = File::open(&path)?;
        let mut schema = String::new();
        f.read_to_string(&mut schema)?;

        println!("Running migration for: {:?}", &path);
        conn.execute_batch(&schema)?;
        println!("Success!");

        Ok(())
    }

    pub fn init_database(&self) -> CommandResult<()> {
        let path = Path::new("./chumper.db3");
        if path.exists() {
            remove_file(path)?;
        }
        let conn = self.get_connection()?;
        let paths = get_ext_from("./db".to_owned(), ".sql".to_owned());

        for path in paths {
            self.run_migration(&conn, path)?;
        }

        Ok(())
    }
}
