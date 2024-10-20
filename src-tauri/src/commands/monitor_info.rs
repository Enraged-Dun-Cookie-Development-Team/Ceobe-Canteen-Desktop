use tauri::command;




#[command]
pub fn message_beep() {
    #[cfg(windows)]
    {
        use windows::Win32::{
            System::Diagnostics::Debug::MessageBeep, UI::WindowsAndMessaging::MB_ICONSTOP,
        };
        unsafe {
            let _ = MessageBeep(MB_ICONSTOP);
        }
    }
}
