pub use error::{Error, Result};
pub use models::*;

// #[cfg(desktop)]
// mod desktop;
// #[cfg(mobile)]
// mod mobile;

mod commands;
mod error;
mod manager;
mod models;
mod request_models;

pub use manager::VersionManager;
// 保留为2.0迁移实现
// #[cfg(desktop)]
// use desktop::VersionManager;
// #[cfg(mobile)]
// use mobile::VersionManager;

// /// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the version-manager APIs.
// pub trait VersionManagerExt<R: Runtime> {
//   fn version_manager(&self) -> &VersionManager<R>;
// }
//
// impl<R: Runtime, T: Manager<R>> crate::VersionManagerExt<R> for T {
//   fn version_manager(&self) -> &VersionManager<R> {
//     self.state::<VersionManager<R>>().inner()
//   }
// }

// /// Initializes the plugin.
// pub fn init<R: Runtime>() -> TauriPlugin<R> {
//   Builder::new("version-manager")
//     .invoke_handler(tauri::generate_handler![commands::ping])
//     .setup(|app, api| {
//       #[cfg(mobile)]
//       let version_manager = mobile::init(app, api)?;
//       #[cfg(desktop)]
//       let version_manager = desktop::init(app, api)?;
//       app.manage(version_manager);
//       Ok(())
//     })
//     .build()
// }
