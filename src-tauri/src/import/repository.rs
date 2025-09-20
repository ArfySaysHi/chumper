use super::CoreData;
use crate::error;
use crate::{import::YamlImportable, shared::get_ext_from};
use rusqlite::{Connection, Result};
use serde::de::DeserializeOwned;
use serde_yml;
use std::fs::File;
use std::io::Read;

pub fn import_yaml_file<T>(
    db: &mut Connection,
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

pub fn fetch_ordered_yaml() -> Vec<String> {
    let core_data_path: String = "./core_data/".to_string();
    let core_data_paths = get_ext_from(core_data_path, ".yaml".to_string());
    log::info!("Found yaml paths: {:#?}", &core_data_paths);

    core_data_paths
}

pub fn import_all(connection: &mut Connection, paths: Vec<String>) -> error::Result<()> {
    for path in paths.iter() {
        log::info!("Importing yaml from: {}", path);
        let mut f = File::open(&path)?;
        let mut raw = String::new();
        f.read_to_string(&mut raw)?;
        let items: Vec<CoreData> = serde_yml::from_str(&raw)?;

        for item in items {
            log::debug!("{:#?}", &item);
            item.insert_into_db(connection)?;
        }
    }

    Ok(())
}
