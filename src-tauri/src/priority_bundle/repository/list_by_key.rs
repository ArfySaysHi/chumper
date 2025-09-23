use super::list_priority_bundles;
use crate::error::Result;
use crate::priority_bundle::repository::PriorityBundleListParams;
use crate::priority_bundle::PriorityBundle;
use crate::shared::{group_by_key, Grade};
use rusqlite::Connection;
use std::collections::HashMap;

pub fn list_by_grade(
    connection: &Connection,
    params: &PriorityBundleListParams,
) -> Result<HashMap<Grade, Vec<PriorityBundle>>> {
    let bundles = list_priority_bundles(connection, params)?;
    let map: HashMap<Grade, Vec<PriorityBundle>> =
        group_by_key(bundles, |pb: &PriorityBundle| Some(pb.grade));

    Ok(map)
}
