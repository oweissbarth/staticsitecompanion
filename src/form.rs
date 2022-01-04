use actix_web::{post, get, HttpResponse, Responder, web};

#[post("/form/{form_id}")]
async fn send_form(web::Path(form_id): web::Path<String>, req_body: String) -> impl Responder{
    let body = format!("form id : {} body: {}", form_id, req_body);
    HttpResponse::Ok().body(body)
}
