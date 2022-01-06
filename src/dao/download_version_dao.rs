use super::Table;
use super::DownloadVersion;


impl<'c> Table<'c, DownloadVersion> {
    pub async fn get_download_by_id(&self, downloadable_id: &str, version: &Option<String>) -> Result<DownloadVersion, sqlx::Error> {
        println!("{}", downloadable_id);
        match version{
            None => {
                println!("no version number");
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
            },
            Some(version) =>{
                println!("got version number");
                sqlx::query_as(
                    r#"
                    SELECT `id`, `version_number`, `resource_url`, `download_id`, `active`
                    FROM `download_version`
                    WHERE `download_id` = ?
                    AND `version_number` = ?
                "#,
                )
                .bind(downloadable_id)
                .bind(version)
                .fetch_one(&*self.pool)
                .await
            }
        }
    }
}