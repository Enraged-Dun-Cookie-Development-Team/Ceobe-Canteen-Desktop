use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    App, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayHandle, SystemTrayMenu,
};
use tracing::info;

const CEOBE_SYSTEM_TRAY: &str = "CEOBE_SYSTEM_TRAY";
pub static  CAN_OPEN_MAIN: AtomicBool = AtomicBool::new(false);
pub fn new_system_tray(app: &mut App) -> tauri::Result<SystemTrayHandle> {
    let handle = app.handle();
    SystemTray::new()
        .with_id(CEOBE_SYSTEM_TRAY)
        .with_tooltip("小刻食堂持续蹲饼中")
        .on_event(move |event| match event {
            SystemTrayEvent::DoubleClick { .. } | SystemTrayEvent::LeftClick { .. } => {
                let can_open = CAN_OPEN_MAIN.load(Ordering::Acquire);
                info!("Current [{}] open the Main", if can_open{"can"}else{"can't"});
                if  can_open{
                    let window = handle.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { tray_id, id, .. } => {
                println!("{tray_id}:{id}");
                if id == "quit" {
                    handle.exit(0)
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
        .add_item(CustomMenuItem::new("quit", "退出小刻食堂"))
}
