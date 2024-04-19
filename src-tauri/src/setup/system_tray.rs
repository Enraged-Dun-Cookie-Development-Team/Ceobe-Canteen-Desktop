use crate::setup::single_instance::BeforeExit;
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
                if id == "quit" {
                    handle.graceful_exit(0)
                }
            }
            _ => {}
        })
        .with_menu(create_system_tray_menu())
        .build(app)
}

pub fn create_system_tray_menu() -> SystemTrayMenu {
    let menu = SystemTrayMenu::new()
        //.add_submenu(SystemTraySubmenu::new("检测更新", SystemTrayMenu::new()))
        //.add_item(CustomMenuItem::new("check-update", "检查更新"))
        .add_item(CustomMenuItem::new("quit", "退出小刻食堂"));

    #[cfg(not(target_os = "windows"))]
    let menu = menu.add_item(CustomMenuItem::new("open", "打开首页"));
    menu
}
