use crate::error::Result;
use rusqlite::Connection;

pub trait YamlImportable {
    fn insert_into_db(&self, connection: &Connection) -> Result<i64>;
}
