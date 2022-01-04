use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadLogEntry {
    pub id: Uuid,
    pub download_version_id: Uuid,
    pub datetime: DateTime<Utc>,
    pub user_agent: Option<String>,
    pub referrer: Option<String>
}


impl FromRow<'_, MySqlRow> for DownloadLogEntry {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DownloadLogEntry {
            id: Uuid::parse_str(row.get("id")).unwrap(),
            download_version_id:  Uuid::parse_str(row.get(1)).unwrap(),
            datetime: row.get("datetime"),
            user_agent: row.get("user_agent"),
            referrer: row.get("referrer")
        })
    }
}