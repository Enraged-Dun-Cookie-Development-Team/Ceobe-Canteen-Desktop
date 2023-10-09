
use serde::{Deserialize, Serialize, Serializer};

use tauri::{AppHandle, command, Window};
use tracing::instrument;

use url::Url;
use crate::commands::message_beep;


#[derive(Debug, thiserror::Error)]
pub enum NotifyError {
    #[error(transparent)]
    TauriApi(#[from] tauri::api::Error),
    #[cfg(windows)]
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[cfg(windows)]
    #[error(transparent)]
    Toast(#[from] winrt_toast::WinToastError),
}

impl Serialize for NotifyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.to_string().serialize(serializer)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    title: String,
    body: String,
    has_sound: bool,
    image_url: Option<Url>,
}
#[command]
#[instrument(name = "SendToastNotify", skip(app,window), err)]
pub fn send_system_notification(
    window: Window,
    app: AppHandle,
    payload: NotificationPayload,
) -> Result<(), NotifyError> {
    #[cfg(windows)]
    {
        use tracing::{error, info};
        use std::sync::OnceLock;
        use winrt_toast::{Toast,ToastManager,Text,Image,};
        static MANAGER:OnceLock<ToastManager> = OnceLock::new();
        let manager = MANAGER.get_or_init(||
        ToastManager::new(app.config().tauri.bundle.identifier.as_str())
        );

        let mut toast = Toast::new();
        toast.launch("click:cookie");
        toast.text1(payload.title);
        toast.text2(Text::new(payload.body));
        if let Some(img) = payload.image_url {
            toast.image(1, Image::new(img));
        }

        manager.show_with_callbacks(
            &toast,
            Some(Box::new(move |launch| {
                let re = launch
                    .map_err(NotifyError::from)
                    .and_then(|_| window.show().map_err(Into::into));
                match re {
                    Ok(_) => {}
                    Err(err) => {
                        error!(Action="Respond To User Select", Error = %err);
                    }
                };
            })),
            Some(Box::new(|reason| match reason {
                Ok(reason) => {
                    info!(Action="Notify Dismiss",reason= ?reason);
                }
                Err(err) => {
                    error!(Action="Notify Dismiss", Error = %err);
                }
            })),
            Some(Box::new(|err| {error!(Action="Notify Failure", Error = %err);}) as _),
        )?;
        if payload.has_sound {
            message_beep()
        }
    }
    #[cfg(not(windows))]
    {
        let mut notify =
            tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier)
                .title(payload.title)
                .body(payload.body);
        if payload.has_sound {
            notify = notify.sound(tauri::api::notification::Sound::Default)
        }
        notify.show()?;
    }

    Ok(())
}
