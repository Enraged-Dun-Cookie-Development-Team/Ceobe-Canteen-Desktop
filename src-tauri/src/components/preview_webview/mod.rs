use tauri::Manager;

use super::ComponentTrait;

#[macro_use]
pub mod commands;
mod set_up;

const PREVIEW_WEBVIEW_LABEL: &'static str = "PreviewWebview";
const MAIN_WINDOWS: &'static str = "main";
const MAIN_WEBVIEW_LABEL: &str = "MainWebview";



pub struct PreviewWebviewComponent;

impl ComponentTrait for PreviewWebviewComponent {
    fn setup(app:&mut tauri::App)->Result<(), Box<dyn std::error::Error>> {
        set_up::setup_main_windows(app.app_handle(), &app.config().app)?;

        Ok(())
    }
}

