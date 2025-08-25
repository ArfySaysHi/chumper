use crate::{character::Character, error::Result, metatype::types::Metatype};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum CoreData {
    Character(Character),
    Metatype(Metatype),
}

impl YamlImportable for CoreData {
    type Output = CoreData;
    fn insert_into_db(&self, connection: &mut Connection) -> Result<Self::Output> {
        match self {
            CoreData::Character(v) => v.insert_into_db(connection).map(CoreData::Character),
            CoreData::Metatype(v) => v.insert_into_db(connection).map(CoreData::Metatype),
        }
    }
}

pub trait YamlImportable {
    type Output;
    fn insert_into_db(&self, connection: &mut Connection) -> Result<Self::Output>
    where
        Self: Sized;
}
