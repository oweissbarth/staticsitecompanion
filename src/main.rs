use actix_web::{ web, App,  HttpServer};
use actix_web::middleware::Logger;
use staticsitecompanion::config::Config;
use staticsitecompanion::dao::Database;
use staticsitecompanion::{controller, AppState};
use std::sync::{Arc, Mutex};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        let config_file: &'static str = "config.json";
        let config = Config::from_file(config_file);
        println!("Using configuration file from {0}", config_file);

        let db_context = Database::new(&config.get_database_url()).await;
        println!("Connected to database: {0}", config.get_database_url());

        let app_state = web::Data::new(AppState {
                connections: Mutex::new(0),
                context: Arc::new(db_context),
            });
        
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

        let app = HttpServer::new(move || {
        App::new()
                .wrap(Logger::default())
                .app_data(app_state.clone())
                .configure(controller::init_downloadable_controller)
        })
        .bind(config.get_app_url())?;
        println!("Listening on: {0}", config.get_app_url());

        app.run().await
}

