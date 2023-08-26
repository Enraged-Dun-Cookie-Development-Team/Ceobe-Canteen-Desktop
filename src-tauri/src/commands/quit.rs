use tauri::{command, AppHandle};

#[command]
pub fn quit(app: AppHandle) {
    app.exit(0)
}
