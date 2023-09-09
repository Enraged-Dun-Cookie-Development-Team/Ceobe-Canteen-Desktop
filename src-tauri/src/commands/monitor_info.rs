use serde::Serialize;

use tauri::{command, PhysicalPosition, PhysicalSize, Window};

#[derive(Debug, Clone, Serialize)]
pub struct MonitorInfo {
    work_space: PhysicalSize<u32>,
    left_top: PhysicalPosition<i32>,
}

#[command]
pub fn get_monitor_info(window: Window) -> tauri::Result<MonitorInfo> {
    #[cfg(not(windows))]
    {
        let window = _window;
        let monitor = window
            .primary_monitor()?
            .expect("Cannot found Primary Monitor");

        Ok(MonitorInfo {
            work_space: *monitor.size(),
            left_top: *monitor.position(),
        })
    }
    #[cfg(windows)]
    {
        use std::mem::size_of;
        use windows::Win32::Graphics::Gdi::{
            GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTOPRIMARY,
        };

        let hwnd = window.hwnd()?;
        let mut monitor_info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };

        unsafe {
            let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY);
            GetMonitorInfoW(monitor, &mut monitor_info);
        };
        let rect = monitor_info.rcWork;

        Ok(MonitorInfo {
            work_space: PhysicalSize::new(
                (rect.right - rect.left) as u32,
                (rect.bottom - rect.top) as u32,
            ),
            left_top: PhysicalPosition::new(rect.left, rect.top),
        })
    }
}

#[command]
pub fn message_beep() {
    #[cfg(windows)]
    {
        use windows::Win32::{
            System::Diagnostics::Debug::MessageBeep, UI::WindowsAndMessaging::MB_ICONSTOP,
        };
        unsafe {
            MessageBeep(MB_ICONSTOP.0);
        }
    }
}
