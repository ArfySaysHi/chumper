use rusqlite::{Connection, Result};

pub fn query_vec<T, F, P>(conn: &Connection, sql: &str, params: P, mut mapper: F) -> Result<Vec<T>>
where
    F: FnMut(&rusqlite::Row) -> rusqlite::Result<T>,
    P: rusqlite::Params,
{
    let mut stmt = conn.prepare(sql)?;
    let items = stmt
        .query_map(params, |row| mapper(row))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(items)
}
