use std::{env, io};
use std::fs::create_dir_all;
use std::path::PathBuf;
use once_cell::sync::OnceCell;
use tauri::api::path::{app_config_dir, config_dir};
use tauri::AppHandle;

pub static APP_NAME: OnceCell<String> = OnceCell::new();

pub fn get_app_name(app: AppHandle) -> &'static str {
    APP_NAME.get_or_init(
        {
            move || {
                let config = &app.config().package;
                format!("{}",
                        config.product_name.as_deref().unwrap_or("CeobeCanteen"),
                )
            }
        })
}

pub static CURRENT_EXE: OnceCell<PathBuf> = OnceCell::new();

pub fn get_current_exe()->io::Result<&'static PathBuf>{
    let current_exe = CURRENT_EXE.get_or_try_init(
        || env::current_exe()
    )?;
    Ok(current_exe)
}

pub static CONFIG_DIR:OnceCell<PathBuf> = OnceCell::new();

pub fn get_config_dir(app:AppHandle)->&'static PathBuf{
    CONFIG_DIR.get_or_init(move|| {
        let path = app_config_dir(&app.config()).expect("Platform not support");
        create_dir_all(&path).expect("Cannot Create Config Dir");
        path
    }

    )
}
