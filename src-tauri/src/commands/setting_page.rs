use tauri::{command, AppHandle, Manager, WindowEvent};
use tracing::error;

#[command]
pub async fn open_setting_page(handle: AppHandle) -> tauri::Result<()> {
    let setting_page = match handle.get_window("settings") {
        Some(window) => window,
        None => {
            return Err(tauri::Error::FailedToExecuteApi(tauri::api::Error::Dialog(
                "Settings window not found".to_string(),
            )))
        }
    };
    // 居主页面之中
    setting_page.center()?;
    setting_page.show()?;
    setting_page.set_focus()?;
    setting_page.clone().on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            match setting_page.hide() {
                Ok(_) => (),
                Err(e) => {
                    error!("Failed to hide settings window: {}", e);
                }
            };
        }
    });
    Ok(())
}
