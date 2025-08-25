use crate::{error::Result, import};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use std::path::Path;
use tauri::{App, AppHandle, Manager};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Pool<SqliteConnectionManager>,
    pub app_handle: AppHandle,
}

impl AppState {
    pub fn new(db_path: &Path, app: &App) -> Result<Self> {
        log::info!("Establishing AppState...");
        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::builder().max_size(15).build(manager)?;

        {
            let mut connection = Connection::open(db_path)?;
            crate::database::repository::init_database(&mut connection)?;

            let yaml_paths = import::repository::fetch_ordered_yaml();
            import::repository::import_all(&mut connection, yaml_paths)?;
        }

        Ok(Self {
            db_pool: pool,
            app_handle: app.app_handle().clone(),
        })
    }
}
