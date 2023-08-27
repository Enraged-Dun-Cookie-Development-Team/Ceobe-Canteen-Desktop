// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod listeners;
mod request_client;
mod setup;
mod single_instance;
mod state;
mod storage;
use std::thread::spawn;
use tauri::{generate_context, Builder, Manager, WindowEvent};

use crate::commands::{
    auto_launch_setting, back_preview, copy_image, get_item, quit, read_detail,
    request_refer_image, send_request, set_auto_launch, set_item,
};
use crate::setup::logger::init_logger;
use crate::setup::system_tray::new_system_tray;
use crate::single_instance::{run_sev, try_start};

fn main() {
    init_logger();
    if let Ok(true) | Err(_) = try_start() {
        let builder = Builder::default()
            .setup(|app| {
                // single instance
                let main_window = app.get_window("main").expect("cannot found main window");
                spawn(move || run_sev(main_window));

                new_system_tray(app)?;
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
                auto_launch_setting,
                get_item,
                set_item,
                send_request,
                quit,
                back_preview
            ]);

        let app = builder
            .build(generate_context!())
            .expect("Create App Failure");
        app.run(|_, _| {});
    } else {
        println!("others start")
    }
}
