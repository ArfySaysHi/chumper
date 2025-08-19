use crate::import::YamlImportable;
use rusqlite::{Connection, Result};
use serde::de::DeserializeOwned;
use serde_yml;

pub fn import_yaml_file<T>(
    db: &Connection,
    file_path: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned + YamlImportable,
{
    let contents = std::fs::read_to_string(file_path)?;
    let items: Vec<T> = serde_yml::from_str(&contents)?;

    for item in &items {
        item.insert_into_db(db)?;
    }

    Ok(items)
}
