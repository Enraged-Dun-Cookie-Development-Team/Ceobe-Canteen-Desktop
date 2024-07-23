use tauri::{
    App, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayHandle, SystemTrayMenu,
};

const CEOBE_SYSTEM_TRAY: &str = "CEOBE_SYSTEM_TRAY";

pub fn new_system_tray(app: &mut App) -> tauri::Result<SystemTrayHandle> {
    let handle = app.handle();
    SystemTray::new()
        .with_id(CEOBE_SYSTEM_TRAY)
        .with_tooltip("小刻食堂持续蹲饼中")
        .on_event(move |event| match event {
            SystemTrayEvent::DoubleClick { .. } | SystemTrayEvent::LeftClick { .. } => {
                let window = handle.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => {
                println!("{tray_id}:{id}");
                match id.as_str() {
                    "quit" => handle.exit(0),
                    "display-switch" => {
                        let window = handle.get_window("main").unwrap();
                        if window.is_visible().unwrap() {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    },
                    _ => {}
                }
            }
            _ => {}
        })
        .with_menu(create_system_tray_menu())
        .build(app)
}

pub fn create_system_tray_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        //.add_submenu(SystemTraySubmenu::new("检测更新", SystemTrayMenu::new()))
        //.add_item(CustomMenuItem::new("check-update", "检查更新"))
        .add_item(CustomMenuItem::new("display-switch", "显隐主窗口"))
        .add_item(CustomMenuItem::new("quit", "退出"))
}
