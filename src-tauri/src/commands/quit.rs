use tauri::{AppHandle, command};

#[command]
pub fn quit(app:AppHandle){
    app.exit(0)
}