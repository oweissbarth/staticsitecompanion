use super::Table;
use super::User;


impl<'c> Table<'c, User> {
    pub async fn get_user_by_id(&self, downloadable_id: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT `id`, `version_number`, `resource_url`, `download_id`, `active`
            FROM `download_version`
            WHERE `download_id` = ?
            AND `active` = 1
            ORDER BY `version_number` DESC"#,
        )
        .bind(downloadable_id)
        .fetch_one(&*self.pool)
        .await
    }
}