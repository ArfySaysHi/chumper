use crate::{
    character::Character, error::Result, metatype::types::Metatype, quality::Quality, skill::Skill,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum CoreData {
    Character(Character),
    Metatype(Metatype),
    Quality(Quality),
    Skill(Skill),
}

impl YamlImportable for CoreData {
    type Output = CoreData;
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output> {
        match self {
            CoreData::Character(v) => v.insert_into_db(connection).map(CoreData::Character),
            CoreData::Metatype(v) => v.insert_into_db(connection).map(CoreData::Metatype),
            CoreData::Quality(v) => v.insert_into_db(connection).map(CoreData::Quality),
            CoreData::Skill(v) => v.insert_into_db(connection).map(CoreData::Skill),
        }
    }
}

pub trait YamlImportable {
    type Output;
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output>
    where
        Self: Sized;
}
