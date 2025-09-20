use crate::{
    error::Result, metatype::types::Metatype, priority_bundle::PriorityBundle, quality::Quality,
    skill::Skill,
};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum CoreData {
    Metatype(Metatype),
    Quality(Quality),
    Skill(Skill),
    PriorityBundle(PriorityBundle),
}

impl YamlImportable for CoreData {
    type Output = CoreData;
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output> {
        log::debug!("wawa: {:#?}", &self);
        match self {
            CoreData::Metatype(v) => v.insert_into_db(connection).map(CoreData::Metatype),
            CoreData::Quality(v) => v.insert_into_db(connection).map(CoreData::Quality),
            CoreData::Skill(v) => v.insert_into_db(connection).map(CoreData::Skill),
            CoreData::PriorityBundle(v) => {
                v.insert_into_db(connection).map(CoreData::PriorityBundle)
            }
        }
    }
}

pub trait YamlImportable {
    type Output;
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output>
    where
        Self: Sized;
}
