
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Clone)]
struct AppConfig {
    url: String,
    port: u16,
}
#[derive(Deserialize, Clone)]
struct DaoConfig {
    user: String,
    password: String,
    address: String,
    database: String,
}

#[derive(Deserialize, Clone)]
struct SendgridConfig {
    api_key: String,
    api_url: String,
    template_id: String,
    from_email: String,
    from_name: String
}

#[derive(Deserialize, Clone)]
pub struct Config {
    app: AppConfig,
    dao: DaoConfig,
    sendgrid: SendgridConfig
}

impl Config {
    pub fn from_file(path: &'static str) -> Self {
        let config = fs::read_to_string(path).unwrap();
        serde_json::from_str(&config).unwrap()
    }

    pub fn get_app_url(&self) -> String {
        format!("{0}:{1}", self.app.url, self.app.port)
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "mysql://{0}:{1}@{2}/{3}",
            self.dao.user, self.dao.password, self.dao.address, self.dao.database
        )
    }

    pub fn get_sendgrid_api_key(&self)-> String {
        self.sendgrid.api_key.clone()
    }

    pub fn get_sendgrid_url(&self)-> String {
        self.sendgrid.api_url.clone()
    }
    pub fn get_template_id(&self)-> String {
        self.sendgrid.template_id.clone()
    }

    pub fn get_from_email(&self) -> String {
        self.sendgrid.from_email.clone()
    }

    pub fn get_from_name(&self) -> String {
        self.sendgrid.from_name.clone()
    }

    pub fn heartbeat(&self) -> bool{
        return true;
    }
}