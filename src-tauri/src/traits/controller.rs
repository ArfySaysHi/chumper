use crate::controllers::{character_controller, database_controller, metatype_controller};
use character_controller::CharacterController;
use database_controller::DatabaseController;
use metatype_controller::MetatypeController;
use rusqlite::{Connection, Result};

pub trait Connectable {
    fn get_connection(&self) -> Result<Connection>;
}

impl Connectable for CharacterController {
    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }
}

impl Connectable for DatabaseController {
    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }
}

impl Connectable for MetatypeController {
    fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }
}
