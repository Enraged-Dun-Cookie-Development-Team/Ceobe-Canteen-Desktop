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

use crate::commands::{
    auto_launch_setting, back_preview, copy_image, front_logger, get_app_cache_path,
    get_app_config_path, get_item, get_monitor_info, hide_notification, is_debug, message_beep,
    quit, read_detail, request_refer_image, send_request, send_system_notification,
    set_auto_launch, set_item, should_silence,
};
use crate::setup::logger::init_logger;
use crate::setup::system_tray::new_system_tray;
use crate::single_instance::{run_sev, try_start};
use tauri::{generate_context, App, Builder, Context, Manager, Runtime, WindowEvent};
use tauri_plugin_cli::Cli;

fn main() {
    // let context: Context<_> = generate_context!();
    // TODO: shift to plugin
    // let context
    // let log_dir = context().log_dir().expect("Log Dir Not available");
    // init_logger(log_dir).expect("Init Log File failure");
    if let Ok(true) | Err(_) = try_start() {
        let builder = Builder::default()
            .plugin(tauri_plugin_cli::init())
            .plugin(tauri_plugin_shell::init())
            .plugin(tauri_plugin_fs::init())
            .plugin(tauri_plugin_os::init())
            .plugin(tauri_plugin_clipboard_manager::init())
            .plugin(tauri_plugin_notification::init())
            .setup(|app| {
                let window = app.get_window("main").expect("cannot found main window");
                fn get_cli<R: Runtime>(app: &App<R>) -> &Cli<R> {
                    app.state::<Cli<_>>().inner()
                }
                let cli = get_cli(app);
                let args = cli.matches()?.args;
                if let Some(arg) = args.get("hidden") {
                    if arg.occurrences == 0 {
                        window.show()?;
                    }
                }
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
                app.webviews()
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
