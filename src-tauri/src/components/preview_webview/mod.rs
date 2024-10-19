#[macro_use]
mod commands;
mod set_up;

const PREVIEW_WEBVIEW_LABEL: &'static str = "PreviewWebview";
const MAIN_WINDOWS: &'static str = "main";
const MAIN_WEBVIEW_LABEL: &str = "MainWebview";

pub use set_up::preview_webview_init;
