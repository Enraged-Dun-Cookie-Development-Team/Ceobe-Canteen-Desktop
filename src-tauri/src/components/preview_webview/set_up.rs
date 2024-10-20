use super::commands::{hidden_preview_webview, show_preview_webview, update_preview_webview};
use crate::components::preview_webview::{MAIN_WEBVIEW_LABEL, MAIN_WINDOWS};
use std::collections::HashMap;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::utils::config::AppConfig;
use tauri::{
    generate_handler, AppHandle, LogicalPosition, Runtime, WebviewBuilder, WindowBuilder,
    WindowEvent, Wry,
};
pub fn preview_webview_init() -> TauriPlugin<Wry, ()> {
    Builder::new("previewWebview")
        .invoke_handler(generate_handler![
            hidden_preview_webview,
            show_preview_webview,
            update_preview_webview
        ])
        .setup(|app, _api| {
            setup_main_windows(app, &app.config().app)?;

            Ok(())
        })
        .build()
}

fn setup_main_windows<R: Runtime>(app: &AppHandle<R>, config: &AppConfig) -> tauri::Result<()> {
    let config_windows: HashMap<_, _> = config
        .windows
        .iter()
        .map(|w| (w.label.as_str(), w))
        .collect();
    if let Some(window_config) = config_windows.get(MAIN_WINDOWS) {
        let window = WindowBuilder::from_config(app, window_config)?.build()?;

        let webview =
            WebviewBuilder::new(MAIN_WEBVIEW_LABEL, window_config.url.to_owned()).auto_resize();

        window.add_child(webview, LogicalPosition::new(0, 0), window.inner_size()?)?;

        window.on_window_event({
            let event_window = window.clone();
            move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    event_window.hide().unwrap()
                }
            }
        })
    }

    Ok(())
}
