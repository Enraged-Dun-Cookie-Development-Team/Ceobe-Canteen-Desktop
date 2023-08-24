// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hidden_window;

use std::sync::OnceLock;
use base64::Engine;
use tauri::{generate_context, Builder, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent, command};
use tauri::api::http::{Client, ClientBuilder, HttpRequestBuilder, ResponseType};
use hidden_window::init_preview;
fn main() {
    Builder::default().setup(|app| {
        let handle = app.handle();
        SystemTray::new().with_tooltip("小刻食堂持续蹲饼中").on_event(move |event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = handle.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => {
                println!("{tray_id}:{id}");
                if id == "quit" {
                    handle.exit(0)
                }
            }
            _ => {}
        }).with_menu(
            SystemTrayMenu::new().add_item(CustomMenuItem::new("check-update", "检查更新")).add_item(CustomMenuItem::new("quit", "退出小刻食堂")),
        ).build(app)?;
        let handle = app.handle();
        let window = handle.get_window("main").unwrap();

        window.clone().on_window_event(move |event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                window.hide().unwrap();
            }
            _ => {}
        });
        #[cfg(debug_assertions)]
        app.windows().values().into_iter().for_each(|window| window.open_devtools());
        Ok(())
    })
        .invoke_handler(tauri::generate_handler![request_refer_image,init_preview])
        .run(generate_context!()).expect("error while running tauri application");
}

#[command]
async fn request_refer_image(url:& str, refer: & str) -> tauri::Result<String> {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    let client = CLIENT.get_or_init(|| ClientBuilder::new().build()
        .expect("create Client Failure"));


    let builder = HttpRequestBuilder::new("GET",url)?
        .header("Referer", refer)?
        .response_type(ResponseType::Binary)

        ;
    let resp = client.send(builder).await?;
    let payload = resp.bytes().await?;
    let payload = base64::engine::general_purpose::STANDARD.encode(payload.data);
    Ok(payload)

}
