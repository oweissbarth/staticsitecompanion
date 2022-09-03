use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

use super::AppState;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_heartbeat);
}


#[get("/heartbeat")]
async fn get_heartbeat(
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {

    let payload = json!({
        "db": app_state.context.heartbeat().await,
        "config": app_state.config.heartbeat(),
        "web": true,
        "version": VERSION
    });

    if true {
        HttpResponse::Ok().json(payload)
    }else{
        HttpResponse::InternalServerError().json(payload)
    }
}