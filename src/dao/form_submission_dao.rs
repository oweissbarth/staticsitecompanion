use super::Table;
use super::FormSubmission;

use sqlx::mysql::{MySqlQueryResult};

impl<'c> Table<'c, FormSubmission> {

    pub async fn add_form_submission(&self, form_submission: &FormSubmission) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO form_submission (`id`, `form_id`, `content`, `user_agent`, `datetime`)
            VALUES(?, ?, ?, ?, ?)"#,
            &form_submission.id.hyphenated(),
            &form_submission.form_id.hyphenated(),
            &form_submission.content,
            &form_submission.user_agent,
            &form_submission.datetime
        )
        .execute(&*self.pool)
        .await
        .map_err(|e| {
            println!("Failed to execute query: {:?}", e);
            e
        })
    }
}