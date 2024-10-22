use tauri::App;

pub mod preview_webview;

pub trait ComponentTrait {
    fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>>;
}
