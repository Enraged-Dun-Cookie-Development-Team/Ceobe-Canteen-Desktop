use crate::state::{get_app_name, get_current_exe};
use auto_launch::AutoLaunch;
use serde::{Serialize, Serializer};
use std::io;
use tauri::{command, AppHandle};

#[derive(Debug, thiserror::Error)]
pub enum AutoLaunchError {
    #[error("Io Error: {0}")]
    Io(#[from] io::Error),
    #[error("System String Encode Not Utf8")]
    UnsupportedOsStringEncode,
    #[error("AutoLaunchError :{0}")]
    AutoLaunch(#[from] auto_launch::Error),
}

impl Serialize for AutoLaunchError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

fn fetch_auto_launch(app: AppHandle) -> Result<AutoLaunch, AutoLaunchError> {
    let current_exe = get_current_exe()?;

    let app_name = get_app_name(app);
    let auto = AutoLaunch::new(
        app_name,
        current_exe
            .to_str()
            .ok_or(AutoLaunchError::UnsupportedOsStringEncode)?,
        &["--hidden"],
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
pub fn auto_launch_setting(app: AppHandle) -> Result<bool, AutoLaunchError> {
    let auto = fetch_auto_launch(app)?;
    Ok(auto.is_enabled()?)
}
