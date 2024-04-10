use tauri::{command, AppHandle, Manager, WindowEvent};

#[command]
pub async fn open_setting_page(handle: AppHandle) {
    let setting_page = handle.get_window("settings").unwrap();
    // 居主页面之中
    setting_page.center().unwrap();
    setting_page.show().unwrap();
    setting_page.set_focus().unwrap();
    setting_page
    .clone()
    .on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            setting_page.hide().unwrap();
        }
    });
}

#[command]
pub async fn close_setting_page(handle: AppHandle) {
    let setting_window = handle.get_window("settings").unwrap();
    setting_window.hide().unwrap();
}
