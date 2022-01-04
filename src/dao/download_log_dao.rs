use super::Table;
use super::DownloadLogEntry;

use sqlx::mysql::{MySqlQueryResult};
use sqlx::types::Uuid;


impl<'c> Table<'c, DownloadLogEntry> {

    pub async fn add_download_log_entry(&self, download_log_entry: &DownloadLogEntry) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO download_log (`id`, `download_version_id`, `datetime`, `user_agent`, `referrer`)
            VALUES(?, ?, ?, ?, ?)"#,
        )
        .bind(&download_log_entry.id.to_hyphenated())
        .bind(&download_log_entry.download_version_id.to_hyphenated())
        .bind(&download_log_entry.datetime)
        .bind(&download_log_entry.user_agent)
        .bind(&download_log_entry.referrer)
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            println!("Failed to execute query: {:?}", e);
            e
        })
    }

    pub async fn get_download_count(&self, download_id: Uuid) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar(
            r#"
            SELECT COUNT(*) as count FROM download_log
            JOIN download_version 
            ON download_log.download_version_id=download_version.id 
            WHERE download_version.download_id=?"#,
        )
        .bind(&download_id.to_hyphenated())
        .fetch_one(&*self.pool)
        .await
    }


    
}