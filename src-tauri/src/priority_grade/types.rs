use super::repository::create_priority_grade;
use crate::error::Result;
use crate::import::YamlImportable;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityGrade {
    pub grade: String,
}

impl YamlImportable for PriorityGrade {
    type Output = PriorityGrade;
    fn insert_into_db(&self, connection: &mut Connection) -> Result<Self::Output> {
        create_priority_grade(connection, &self)
    }
}
