use super::{DownloadVersion, Download, DownloadLogEntry, User};
use sqlx::mysql::{MySqlRow, MySqlPoolOptions};
use sqlx::{FromRow, MySqlPool};
use std::sync::Arc;

pub struct Table<'r, T>
    where
        T: FromRow<'r, MySqlRow>,
{
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'r MySqlRow) -> Result<T, sqlx::Error>,
}

impl<'r, T> Table<'r, T>
    where
        T: FromRow<'r, MySqlRow>,
{
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
        }
    }
}


pub struct Database<'r> {
    pub users: Arc<Table<'r, User>>,
    pub downloads: Arc<Table<'r, Download>>,
    pub download_versions: Arc<Table<'r, DownloadVersion>>,
    pub download_logs: Arc<Table<'r, DownloadLogEntry>>
}

impl Database<'_> {
    pub async fn new(sql_url: &str) -> Database<'_> {
        let pool = MySqlPoolOptions::new()
            .max_connections(8) // TODO: pass in the pool connection count
            .connect(sql_url)
            .await
            .unwrap();
        let pool = Arc::new(pool);

        Database {
            users: Arc::from(Table::new(pool.clone())),
            downloads: Arc::from(Table::new(pool.clone())),
            download_versions: Arc::from(Table::new(pool.clone())),
            download_logs: Arc::from(Table::new(pool.clone()))
        }
    }
}