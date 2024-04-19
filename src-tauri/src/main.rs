// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ffi::CString;

use tauri::api::path::app_log_dir;
use tauri::{generate_context, Builder, Context, Manager, WindowEvent};
use windows::core::PCSTR;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_ICONERROR, MB_OK};

use crate::commands::{
    auto_launch_setting, back_preview, copy_image, front_logger, get_app_cache_path,
    get_app_config_path, get_item, get_monitor_info, hide_notification, is_debug, message_beep,
    quit, read_detail, request_refer_image, send_request, send_system_notification,
    set_auto_launch, set_item, should_silence,
};
use crate::setup::logger::init_logger;
use crate::setup::single_instance::{BeforeExit, SingleInstanceManager};
use crate::setup::system_tray::new_system_tray;

mod commands;
mod listeners;
mod request_client;
mod setup;
mod state;
mod storage;

fn main() {
    let context: Context<_> = generate_context!();
    let log_dir = app_log_dir(context.config()).expect("Log Dir Not available");
    init_logger(log_dir).expect("Init Log File failure");
    if let Ok(fd) = SingleInstanceManager.check_instance(context.config()) {
        let builder = Builder::default()
            .manage(fd)
            .setup(|app| {
                let window = app.get_window("main").expect("cannot found main window");
                let args = app.get_cli_matches()?.args;
                if let Some(arg) = args.get("hidden") {
                    if arg.occurrences == 0 {
                        window.show()?;
                    }
                }
                ctrlc_async::set_handler({
                    let local_app = app.handle().clone();
                    move || local_app.graceful_exit(0)
                })
                .expect("Set Exit Handler Error");
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
        println!("others start");
        #[cfg(target_os = "windows")]
        {
            let message = format! { "\"CeobeCanteen\" is already running.\n\
             If there is indeed no running \"CeobeCanteen\", \
             please delete the file located in [{:?}] and restart the application.",
            SingleInstanceManager.get_lck_file(context.config())
            };
            let message = CString::new(message).expect("Bad String");
            let title = CString::new("Error").expect("Bad String");

            unsafe {
                MessageBoxA(
                    None,
                    PCSTR::from_raw(message.as_ptr() as _),
                    PCSTR::from_raw(title.as_ptr() as _),
                    MB_OK | MB_ICONERROR,
                );
            }
        }
    }
}
