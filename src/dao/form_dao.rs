use super::Form;
use super::Table;

impl<'c> Table<'c, Form> {
    pub async fn get_form_by_id(&self, form_id: &str) -> Result<Form, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `name`, `user`, `redirect_success`, `redirect_failure`, `notify_email`
            FROM `form`
            WHERE `id` = ?"#,
        )
        .bind(form_id)
        .fetch_one(&*self.pool)
        .await
    }
}