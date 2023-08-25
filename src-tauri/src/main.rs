// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod preview_page;
mod single_instance;
mod clipboard;
mod auto_launch;

use clipboard::copy_image;
use base64::Engine;
use std::sync::OnceLock;
use std::thread::spawn;
use tauri::{
    command, generate_context, Builder, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowEvent,
};

use crate::single_instance::{run_sev, try_start};
use preview_page::read_detail;
use tauri::api::http::{Client, ClientBuilder, HttpRequestBuilder, ResponseType};
use crate::auto_launch::{set_auto_launch,auto_launch_setting};
fn main() {
    if let Ok(true) | Err(_) = try_start() {
        Builder::default()
            .setup(|app| {
                // single instance
                let main_window = app.get_window("main").expect("cannot found main window");
                spawn(move || run_sev(main_window));

                let handle = app.handle();
                SystemTray::new()
                    .with_tooltip("小刻食堂持续蹲饼中")
                    .on_event(move |event| match event {
                        SystemTrayEvent::DoubleClick { .. } | SystemTrayEvent::LeftClick { .. } => {
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
                    })
                    .with_menu(
                        SystemTrayMenu::new()
                            .add_item(CustomMenuItem::new("check-update", "检查更新"))
                            .add_item(CustomMenuItem::new("quit", "退出小刻食堂")),
                    )
                    .build(app)?;
                let handle = app.handle();
                let window = handle.get_window("main").unwrap();

                window.clone().on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        window.hide().unwrap();
                    }
                });
                #[cfg(debug_assertions)]
                app.windows()
                    .values()
                    .for_each(|window| window.open_devtools());

                Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                request_refer_image,
                read_detail,
                copy_image,
                set_auto_launch,
                auto_launch_setting
            ])
            .run(generate_context!())
            .expect("error while running tauri application");
    }
}

#[command]
async fn request_refer_image(url: &str, refer: &str) -> tauri::Result<String> {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    let client =
        CLIENT.get_or_init(|| ClientBuilder::new().build().expect("create Client Failure"));

    let builder = HttpRequestBuilder::new("GET", url)?
        .header("Referer", refer)?
        .response_type(ResponseType::Binary);
    let resp = client.send(builder).await?;
    let payload = resp.bytes().await?;
    let payload = base64::engine::general_purpose::STANDARD.encode(payload.data);
    Ok(payload)
}
