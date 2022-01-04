//use sqlx::types::Uuid;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
}

impl FromRow<'_, MySqlRow> for User {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let r = Uuid::parse_str(row.get("id"));
        Ok(User {
            id: r.unwrap(),
            email: row.get("email")
        })
    }
}