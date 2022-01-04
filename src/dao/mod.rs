use super::model::{Download, DownloadVersion, DownloadLogEntry, User};

pub mod db_context;
mod downloadable_dao;
//mod user_dao;
mod download_log_dao;
mod download_version_dao;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;
//pub type JoinTable<'c, T1, T2> = db_context::JoinTable<'c, T1, T2>;