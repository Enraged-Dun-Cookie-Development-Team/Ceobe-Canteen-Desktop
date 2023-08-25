use std::{env, io};
use std::path::PathBuf;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Serialize, Serializer};
use tauri::{AppHandle, command};

static APP_NAME: OnceCell<String> = OnceCell::new();
static CURRENT_EXE: OnceCell<PathBuf> = OnceCell::new();

#[derive(Debug, thiserror::Error)]
pub enum AutoLaunchError {
    #[error("Io Error: {0}")]
    Io(#[from]io::Error),
    #[error("System String Encode Not Utf8")]
    UnsupportedOsStringEncode,
    #[error("AutoLaunchError :{0}")]
    AutoLaunch(#[from]auto_launch::Error),
}

impl Serialize for AutoLaunchError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.to_string().serialize(serializer)
    }
}

fn fetch_auto_launch(app: AppHandle) -> Result<AutoLaunch, AutoLaunchError> {
    let current_exe = CURRENT_EXE.get_or_try_init(
        || env::current_exe()
    )?;

    let app_name = APP_NAME.get_or_init(
        {
            let app = app.clone();
            move || {
                let config = &app.config().package;
                format!("{}-{}",
                        config.product_name.as_deref().unwrap_or("CeobeCanteen"),
                        config.version.as_deref().unwrap_or("0.0.0"))
            }
        });
    let auto = AutoLaunch::new(
        app_name, current_exe.to_str().ok_or(AutoLaunchError::UnsupportedOsStringEncode)?,
        &[
            "--hidden"
        ],
    );
    Ok(auto)
}

#[command]
pub fn set_auto_launch(app: AppHandle, auto_launch: bool) -> Result<bool, AutoLaunchError> {
    let auto = fetch_auto_launch(app)?;
    if auto_launch {
        auto.enable()?
    } else {
        auto.disable()?
    }

    let enable = auto.is_enabled()?;

    Ok(enable)
}

#[command]
pub fn auto_launch_setting(app:AppHandle)->Result<bool,AutoLaunchError>{
    let auto = fetch_auto_launch(app)?;
    Ok(auto.is_enabled()?)
}