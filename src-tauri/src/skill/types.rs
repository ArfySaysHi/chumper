use crate::error::Result;
use crate::import::YamlImportable;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub attribute: String,
    pub can_default: bool,
    pub skill_group: Option<String>,
    pub specializations: Option<Vec<Specialization>>,
}

impl YamlImportable for Skill {
    type Output = Skill;
    fn insert_into_db(&self, connection: &Connection) -> Result<Self::Output> {
        super::repository::create_skill_group(connection, self)?;
        let s = super::repository::create_skill(connection, self)?;
        super::repository::create_skill_specialization(connection, &s)?;
        Ok(s)
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct SkillGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Specialization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub name: String,
    pub skill_id: Option<i64>,
}
