use std::path::PathBuf;
use tauri::{AppHandle, command};
use crate::state::{get_cache_dir, get_config_dir};

#[command]
pub fn get_app_config_path(app:AppHandle)->PathBuf{
    get_config_dir(app).to_owned()
}
#[command]
pub fn get_app_cache_path(app:AppHandle)->PathBuf{
    get_cache_dir(app).to_owned()
}