use tauri::{command, Window};

#[allow(unused_variables)]
#[command]
pub fn hide_notification(window: Window) -> tauri::Result<()> {
    #[cfg(windows)]
    {
        use windows::Win32::Graphics::Gdi::UpdateWindow;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE, WS_EX_APPWINDOW, WS_EX_TOOLWINDOW,
        };
        let hwnd = window.hwnd()?;
        let mut style = unsafe { GetWindowLongPtrW(hwnd, GWL_EXSTYLE) };
        style |= !WS_EX_APPWINDOW.0 as isize;
        style &= WS_EX_TOOLWINDOW.0 as isize;
        unsafe { SetWindowLongPtrW(hwnd, GWL_EXSTYLE, style) };
        unsafe {
            UpdateWindow(hwnd);
        }
    }

    Ok(())
}
