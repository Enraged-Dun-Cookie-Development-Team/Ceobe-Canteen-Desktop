//! 根据 https://v2.tauri.app/start/migrate/from-tauri-1/#migrate-to-tray-icon-module 迁移

use tauri::menu::{Menu, MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::{App, Manager, Runtime, UserAttentionType};

const CEOBE_SYSTEM_TRAY: &str = "CEOBE_SYSTEM_TRAY";

pub fn new_system_tray(app: &mut App) -> tauri::Result<TrayIcon> {
    TrayIconBuilder::with_id(CEOBE_SYSTEM_TRAY)
        .tooltip("小刻食堂持续蹲饼中")
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => app.exit(0),
            "display-switch" => {
                let window = app.get_window("main").unwrap();
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            }
            | TrayIconEvent::DoubleClick { .. } => {
                let handle = tray.app_handle();
                let window = handle.get_webview_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
                window
                    .request_user_attention(Some(UserAttentionType::Critical))
                    .unwrap()
            }
            _ => {}
        })
        .menu(&create_system_tray_menu(&app)?)
        .icon(
            app.default_window_icon()
                .cloned()
                .expect("Not Icon for this APP"),
        )
        .build(app)
}

pub fn create_system_tray_menu<R: Runtime>(app: &App<R>) -> tauri::Result<Menu<R>> {
    let menu_open_primary = MenuItemBuilder::with_id("display-switch", "显隐主窗口").build(app)?;
    let menu_exit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    MenuBuilder::new(app)
        .items(&[&menu_open_primary, &menu_exit])
        .build()
}
