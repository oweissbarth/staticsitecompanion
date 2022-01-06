use super::get_user_agent;
use super::AppState;
use actix_web::{post, web, HttpResponse, HttpRequest, Responder, http::header};
use super::FormSubmission;
use sqlx::types::{Uuid, Json};
use sqlx::types::chrono::Utc;
use serde_json::json;
use std::collections::HashMap;
use futures::join;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_form);
}

#[post("/form/{id}")]
async fn submit_form(
    form_fields: web::Form<HashMap<String, String>>,
    req: HttpRequest,
    form_id: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {

    let form = app_state.context.forms.get_form_by_id(&form_id).await;
    
    let fields = form_fields.into_inner();

    let content = Json(fields.clone());

    match form {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(form) => {
            let submission = FormSubmission{
                id: Uuid::new_v4(),
                form_id: form.id,
                content: content.clone(),
                datetime: Utc::now(),
                user_agent: get_user_agent(&req),

            };

            let insert_f = app_state.context.form_submissions.add_form_submission(&submission);

            let payload = json!(
                {
                "template_id": app_state.config.get_template_id(),
                "subject": "This will be replaced by the template",
                "from": {"email": app_state.config.get_from_email(), "name": app_state.config.get_from_name()},
                "content": [{"type": "text/plain", "value": "This will be replaced by the template"}],
                "personalizations": [{"to": [{"email":form.notify_email, "name": "me"}], "dynamic_template_data": {"form_fields": content.clone()}}],
            });

            let client = reqwest::Client::new();
            let sendgrid_url = app_state.config.get_sendgrid_url();
            let sendgrid_api_key = app_state.config.get_sendgrid_api_key();
            let mail_f = client.post(sendgrid_url)
                .json(&payload)
                .header(header::AUTHORIZATION, format!("Bearer {}", sendgrid_api_key))
                .send();
            
            let results = join!(insert_f, mail_f);
            

            match results{
                (Ok(_), Ok(_)) =>HttpResponse::Found().append_header(("Location", form.redirect_success.unwrap())).finish(),
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}