use super::{get_referrer, get_user_agent};
use super::AppState;
use actix_web::{get, web, HttpResponse, HttpRequest, Responder};
use super::DownloadLogEntry;
use sqlx::types::Uuid;
use sqlx::types::chrono::{Utc};
use serde::{Deserialize, Serialize};



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_download);
    cfg.service(get_download_count);
}

#[derive(Deserialize)]
pub struct DownloadRequest{
    version: Option<String>
}

#[get("/download/{id}")]
async fn get_download(
    req: HttpRequest,
    download_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
    query: web::Query<DownloadRequest>
) -> impl Responder {

    let download_version = app_state.context.download_versions.get_download_by_id(&download_id, &query.version).await;

    match download_version {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(download_version) => {
           
            let download = DownloadLogEntry{
                id: Uuid::new_v4(),
                datetime: Utc::now(),
                referrer: get_referrer(&req),
                user_agent: get_user_agent(&req),
                download_version_id: download_version.id
            };

            let r = app_state.context.download_logs.add_download_log_entry(&download).await;
            match r{
                Err(_) => HttpResponse::InternalServerError().finish(),
                Ok(_r) => HttpResponse::Found().append_header(("Location", download_version.resource_url)).finish()
            }
        }
    }
}

#[derive(Serialize)]
struct DownloadStats{
    id: Uuid,
    count: i64
}

#[get("/download/{id}/count")]
async fn get_download_count(
    downloadable_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {

    let downloadable = app_state.context.download_versions.get_download_by_id(&downloadable_id, &None).await;

    match downloadable {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(downloadable) => {
            let download_count = app_state.context.download_logs.get_download_count(downloadable.download_id).await;
            match download_count{
                Err(_) => HttpResponse::InternalServerError().finish(),
                Ok(download_count) => {
                            let stats  = DownloadStats{
                                id: downloadable.download_id,
                                count: download_count.unwrap_or(0)
                            };
                            HttpResponse::Ok().json(stats)
                }
            }
        }
    }
}