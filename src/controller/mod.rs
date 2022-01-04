
use super::AppState;
use std::sync::Mutex;

pub mod downloadable_controller;
use super::model::{DownloadLogEntry};

pub use downloadable_controller::init as init_downloadable_controller;


fn log_request(route: &'static str, connections: &Mutex<u32>) {
    let mut con = connections.lock().unwrap();
    *con += 1;
    println!("{}\n\tconnections: {}", route, con);
}