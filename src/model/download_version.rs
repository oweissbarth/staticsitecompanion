use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
//use sqlx::types::Uuid;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct DownloadVersion {
    pub id: Uuid,
    pub version_number: String,
    pub resource_url: String,
    pub download_id: Uuid,
    pub active: bool
}

impl FromRow<'_, MySqlRow> for DownloadVersion {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(DownloadVersion {
            id:  Uuid::parse_str(row.get("id")).unwrap(),
            version_number: row.get("version_number"),
            resource_url: row.get("resource_url"),
            download_id:  Uuid::parse_str(row.get("download_id")).unwrap(),
            active: row.get("active")
        })
    }
}