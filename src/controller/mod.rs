
use super::AppState;
use actix_web::{HttpRequest, http::header};


pub mod downloadable_controller;
pub mod form_controller;
use super::model::{FormSubmission, DownloadLogEntry};

pub use downloadable_controller::init as init_downloadable_controller;
pub use form_controller::init as init_form_controller;


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