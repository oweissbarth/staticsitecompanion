use super::log_request;
use super::AppState;
use actix_web::{get, web, HttpResponse, HttpRequest, Responder, http::header};
use super::DownloadLogEntry;
use sqlx::types::Uuid;
use sqlx::types::chrono::{Utc};
use serde::{Deserialize, Serialize};



pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_download);
    cfg.service(get_download_count);
}


pub fn get_referrer(req: &HttpRequest) -> Option<String>{
    let referrer_value = req.headers().get(header::REFERER);
    match referrer_value{
        None => None,
        Some(x) => {
            match x.to_str(){
                Ok(str) => Some(String::from(str)),
                Err(_) => None
            }
        }
    }
}

pub fn get_user_agent(req: &HttpRequest) -> Option<String>{
    let user_agent_value = req.headers().get(header::USER_AGENT);
    match user_agent_value{
        None => None,
        Some(x) => {
            match x.to_str(){
                Ok(str) => Some(String::from(str)),
                Err(_) => None
            }
        }
    }
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
    log_request("GET: /downloadable", &app_state.connections);

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
    req: HttpRequest,
    downloadable_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    log_request("GET: /downloadable", &app_state.connections);

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
                        count: download_count
                    };
                    HttpResponse::Ok().json(stats)
                }
            }
        }
    }
}