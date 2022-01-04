mod user;
mod download;
mod download_version;
mod download_log_entry;

pub type User = user::User;
pub type Download = download::Download;
pub type DownloadVersion = download_version::DownloadVersion;
pub type DownloadLogEntry = download_log_entry::DownloadLogEntry;