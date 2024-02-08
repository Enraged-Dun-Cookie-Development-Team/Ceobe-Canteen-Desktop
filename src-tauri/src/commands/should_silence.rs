use tauri::command;

#[command]
pub fn should_silence() -> tauri::Result<bool> {
    #[cfg(windows)]
    {
        use std::mem::size_of;
        use windows::Win32::{
            Foundation::RECT,
            Graphics::Gdi::{
                GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTOPRIMARY,
            },
            UI::WindowsAndMessaging::{
                GetDesktopWindow, GetForegroundWindow, GetShellWindow, GetWindowRect,
            },
        };

        let shell = unsafe { GetShellWindow() };
        let desktop = unsafe { GetDesktopWindow() };
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.0 == 0 || (hwnd == shell || hwnd == desktop) {
            Ok(false)
        } else {
            let mut rect = RECT::default();
            unsafe { GetWindowRect(hwnd, &mut rect) };

            let mut monitor_info = MONITORINFO {
                cbSize: size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            let monitor = unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY) };
            unsafe { GetMonitorInfoW(monitor, &mut monitor_info) };

            let monitor_size = monitor_info.rcMonitor;

            Ok(rect.left == monitor_size.left
                && rect.right == monitor_size.right
                && rect.top == monitor_size.top
                && rect.bottom == monitor_size.bottom)
        }
    }
    #[cfg(not(windows))]
    Ok(false)
}
