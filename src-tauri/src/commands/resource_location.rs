use crate::state::{get_cache_dir, get_config_dir};
use std::path::PathBuf;
use tauri::{command, AppHandle};

#[command]
pub fn get_app_config_path(app: AppHandle) -> PathBuf {
    get_config_dir(&app.config()).to_owned()
}
#[command]
pub fn get_app_cache_path(app: AppHandle) -> PathBuf {
    get_cache_dir(app).to_owned()
}
