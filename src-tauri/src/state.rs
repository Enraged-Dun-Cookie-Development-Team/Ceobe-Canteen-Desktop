use once_cell::sync::OnceCell;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::{env, io};

use tauri::{AppHandle, Manager};
use tracing::{info, instrument};

pub static APP_NAME: OnceCell<String> = OnceCell::new();

#[instrument(skip_all)]
pub fn get_app_name(app: AppHandle) -> &'static str {
    APP_NAME.get_or_init({
        move || {
            let config = &app.config();
            let name = config
                .product_name
                .as_deref()
                .unwrap_or("CeobeCanteen")
                .to_string();
            info!(firstInit = "APP_NAME", value = name);
            name
        }
    })
}

pub static CURRENT_EXE: OnceCell<PathBuf> = OnceCell::new();

#[instrument(skip_all, err)]
pub fn get_current_exe() -> io::Result<&'static PathBuf> {
    let current_exe = CURRENT_EXE.get_or_try_init(|| {
        let exe = env::current_exe()?;
        info!(firstInit = "CURRENT_EXE",value = ?exe);
        Ok::<_, io::Error>(exe)
    })?;
    Ok(current_exe)
}

pub static CONFIG_DIR: OnceCell<PathBuf> = OnceCell::new();

#[instrument(skip_all)]
pub fn get_config_dir(app: AppHandle) -> &'static PathBuf {
    CONFIG_DIR.get_or_init(move || {
        let path = app.path().config_dir().expect("Platform not support");
        create_dir_all(&path).expect("Cannot Create Config Dir");
        info!(firstInit = "CONFIG_DIR", path = ?path);
        path
    })
}

pub static CACHE_DIR: OnceCell<PathBuf> = OnceCell::new();

#[instrument(skip_all)]
pub fn get_cache_dir(app: AppHandle) -> &'static PathBuf {
    CACHE_DIR.get_or_init(move || {
        let path = app.path().cache_dir().expect("Platform not support");
        create_dir_all(&path).expect("Cannot Create Config Dir");
        info!(firstInit = "CACHE_DIR", path = ?path);
        path
    })
}
