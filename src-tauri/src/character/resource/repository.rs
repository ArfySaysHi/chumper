use crate::error::Result;
use rusqlite::{named_params, Connection};

use super::{CreateResourceParams, Resource};

pub fn list_resources(connection: &Connection, character_id: i64) -> Result<Vec<Resource>> {
    let query = format!(
        "SELECT * FROM resources WHERE character_id = {}",
        character_id
    );
    let mut stmt = connection.prepare(&query)?;
    let res = stmt.query_map([], |row| {
        Ok(Resource {
            id: row.get("id")?,
            name: row.get("name")?,
            base_amount: row.get("base_amount")?,
            current_amount: row.get("current_amount")?,
            character_id: row.get("character_id")?,
        })
    })?;

    Ok(res.collect::<rusqlite::Result<Vec<_>>>()?)
}

pub fn create_resource(connection: &Connection, params: CreateResourceParams) -> Result<String> {
    println!("Attempting to create_resource with: {:?}", params);
    let query = "INSERT INTO resources
           (name, base_amount, current_amount, character_id)
         VALUES
           (:name, :base_amount, :current_amount, :character_id)
        ";
    let mut stmt = connection.prepare(&query)?;
    stmt.execute(named_params! {
        ":name": params.name,
        ":base_amount": params.base_amount,
        ":current_amount": params.current_amount,
        ":character_id": params.character_id
    })?;

    println!(
        "Resource ({}) created successfully for character ({})",
        params.name, params.character_id
    );
    Ok("Resource created successfully".to_string())
}
