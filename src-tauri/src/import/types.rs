use crate::error::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub trait YamlImportable {
    fn insert_into_db(&self, connection: &Connection) -> Result<i64>;
}

pub trait YamlSerializable {
    fn from_yaml(yaml: &str) -> Result<Self>
    where
        Self: Sized + for<'de> Deserialize<'de>,
    {
        Ok(serde_yml::from_str(yaml)?)
    }

    fn to_yaml(&self) -> Result<String>
    where
        Self: Serialize,
    {
        Ok(serde_yml::to_string(self)?)
    }
}
