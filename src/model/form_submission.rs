use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use sqlx::types::{Uuid, Json};
use std::collections::HashMap;
use sqlx::types::chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Clone)]
pub struct FormFields{
    pub fields: HashMap<String, String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FormSubmission {
    pub id: Uuid,
    pub form_id: Uuid,
    pub content: Json<HashMap<String, String>>,
    pub user_agent: Option<String>,
    pub datetime: DateTime<Utc>
}

impl FromRow<'_, MySqlRow> for FormSubmission {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(FormSubmission {
            id:  Uuid::parse_str(row.get("id")).unwrap(),
            form_id:  Uuid::parse_str(row.get("user")).unwrap(),
            content: row.get("content"),
            user_agent: row.get("user_agent"),
            datetime: row.get("datetime")
        })
    }
}