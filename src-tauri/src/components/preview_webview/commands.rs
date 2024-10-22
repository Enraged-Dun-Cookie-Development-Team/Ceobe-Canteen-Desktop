use tauri::webview::PageLoadEvent;
use tauri::{
    command, AppHandle, LogicalPosition, LogicalSize, Manager, WebviewBuilder, WebviewUrl, Window,
};
use url::Url;

use crate::components::preview_webview::{MAIN_WINDOWS, PREVIEW_WEBVIEW_LABEL};

#[command]
pub async fn show_preview_webview(
    app: AppHandle,
    target_url: Option<Url>,
    target_position: LogicalPosition<f64>,
    target_size: LogicalSize<f64>,
) -> tauri::Result<()> {
    let window = app.get_window(MAIN_WINDOWS);

    if let Some(main_window) = window {
        // webview exist
        if let Some(preview_webview) = main_window.get_webview(PREVIEW_WEBVIEW_LABEL) {
            preview_webview.set_position(target_position)?;
            preview_webview.set_size(target_size)?;
            if let Some(target_url) = target_url {
                preview_webview.eval(&format!("window.location.replace('{target_url}')"))?;
            }
            preview_webview.show()?;
        } else {
            // no exist,create it
            let preview_webview = WebviewBuilder::new(
                PREVIEW_WEBVIEW_LABEL,
                target_url.map(WebviewUrl::External).unwrap_or_default(),
            )
            .auto_resize()
            .on_page_load(|webview, event| {
                if let PageLoadEvent::Finished = event.event() {
                    webview
                        .eval(include_str!("init_script.js"))
                        .expect("Inject JS Failure")
                }
            });
            let preview_webview =
                main_window.add_child(preview_webview, target_position, target_size)?;

            preview_webview.set_position(target_position)?;
            preview_webview.set_size(target_size)?;

            preview_webview.show()?;
        }
    }

    Ok(())
}
#[command]
pub async fn hidden_preview_webview(app: AppHandle) -> tauri::Result<()> {
    if let Some(preview_webview) = app.get_webview(PREVIEW_WEBVIEW_LABEL) {
        preview_webview.eval("window.location.replace('about:black')")?;
        preview_webview.hide()?;
    }
    Ok(())
}
#[command]
pub async fn update_preview_webview(
    window: Window,
    pos: LogicalPosition<f64>,
    size: LogicalSize<f64>,
) -> tauri::Result<()> {
    if let Some(webview) = window.get_webview(PREVIEW_WEBVIEW_LABEL) {
        webview.set_position(pos)?;
        webview.set_size(size)?;
    }
    Ok(())
}
