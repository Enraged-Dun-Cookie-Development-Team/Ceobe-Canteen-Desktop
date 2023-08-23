// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    generate_context, Builder, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, WindowEvent,
};

fn main() {
    Builder::default()
        .setup(|app| {
            let handle = app.handle();
            SystemTray::new()
                .with_tooltip("小刻食堂持续蹲饼中")
                .on_event(move |event| match event {
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
                })
                .with_menu(
                    SystemTrayMenu::new()
                        .add_item(CustomMenuItem::new("check-update", "检查更新"))
                        .add_item(CustomMenuItem::new("quit", "退出小刻食堂")),
                )
                .build(app)?;
            let handle = app.handle();
            let window = handle.get_window("main").unwrap();

            #[cfg(debug_assertions)]
            window.clone().open_devtools();

            window.clone().on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    window.hide().unwrap();
                }
                _ => {}
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
