// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod listeners;
mod request_client;
mod setup;
mod single_instance;
mod state;
mod storage;

use std::sync::atomic::Ordering;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::spawn;
use tauri::api::path::app_log_dir;
use tauri::{generate_context, Builder, Context, Manager, WindowEvent};
use log::log;
use crate::commands::{
    auto_launch_setting, back_preview, copy_image, front_logger, get_app_cache_path,
    get_app_config_path, get_item, get_monitor_info, hide_notification, is_debug, message_beep,
    quit, read_detail, request_refer_image, send_request, send_system_notification,
    set_auto_launch, set_item, should_silence,
};
use crate::setup::logger::init_logger;
use crate::setup::system_tray::{new_system_tray, CAN_OPEN_MAIN};
use crate::single_instance::{run_sev, try_start};

fn main() {
    let context: Context<_> = generate_context!();
    let log_dir = app_log_dir(context.config()).expect("Log Dir Not available");
    init_logger(log_dir).expect("Init Log File failure");
    if let Ok(true) | Err(_) = try_start() {
        let builder = Builder::default()
            .setup(|app| {
                // silence startup
                let mut need_show = false;
                let window = app.get_window("main").expect("cannot found main window");
                let args = app.get_cli_matches()?.args;
                if let Some(arg) = args.get("hidden") {
                    if arg.occurrences == 0 {
                        need_show = true;
                    }
                }
                // updater
                app.listen_global("updater-exit",{
                    let local_main = window.clone();
                    move |_|{
                        CAN_OPEN_MAIN.store(true, Ordering::Release);
                        if need_show && !local_main.is_visible().unwrap_or_default() {
                            local_main.show().ok();
                        }
                    }
                });

                // single instance
                spawn({
                    let main_window = window.clone();
                    move || run_sev(main_window)
                });

                new_system_tray(app)?;

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
                back_preview,
                front_logger,
                get_monitor_info,
                message_beep,
                get_app_cache_path,
                get_app_config_path,
                hide_notification,
                is_debug,
                send_system_notification,
                should_silence
            ]);

        let app = builder
            .build(generate_context!())
            .expect("Create App Failure");
        app.run(|_, _| {});
    } else {
        println!("others start")
    }
}
