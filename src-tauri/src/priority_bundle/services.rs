use super::{repository, PriorityBundle};
use crate::database::AppState;

pub struct PriorityBundleService;

impl PriorityBundleService {
    pub fn list_all(
        connection: &rusqlite::Connection,
    ) -> crate::error::Result<Vec<PriorityBundle>> {
        repository::list_priority_bundles(connection)
    }
}
