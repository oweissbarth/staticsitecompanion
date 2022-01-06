use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::types::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct Form {
    pub id: Uuid,
    pub name: String,
    pub user: Uuid,
    pub redirect_success: Option<String>,
    pub redirect_failure: Option<String>,
    pub notify_email: Option<String>
}

impl FromRow<'_, MySqlRow> for Form {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Form {
            id:  Uuid::parse_str(row.get("id")).unwrap(),
            name: row.get("name"),
            user:  Uuid::parse_str(row.get("user")).unwrap(),
            redirect_success: row.get("redirect_success"),
            redirect_failure: row.get("redirect_failure"),
            notify_email: row.get("notify_email")
        })
    }
}