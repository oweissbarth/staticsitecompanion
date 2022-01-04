use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
//use sqlx::types::Uuid;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct Download {
    pub id: Uuid,
    pub name: String,
    pub user: Uuid,
}

impl FromRow<'_, MySqlRow> for Download {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Download {
            id:  Uuid::parse_str(row.get("id")).unwrap(),
            name: row.get("name"),
            user:  Uuid::parse_str(row.get("user")).unwrap(),
        })
    }
}