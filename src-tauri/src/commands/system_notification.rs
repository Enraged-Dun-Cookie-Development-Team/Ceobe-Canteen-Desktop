use serde::{Deserialize, Serialize};
use tauri::api::notification::Sound;
use tauri::AppHandle;
use windows::core::InParam;
use windows::Data::Xml::Dom::{IXmlNode, XmlDocument};
use windows::UI::Notifications::{Notification, ToastNotificationManager, ToastTemplateType};
use crate::commands::request_refer_image;

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    title: String,
    icon: Option<String>,
    body: String,
    has_sound: bool,
    image_url: Option<String>,
}

#[cfg(windows)]
pub async fn send_system_notification(app: AppHandle, payload: NotificationPayload) -> tauri::Result<()> {
    #[cfg(windows)]
    {
        let xml = ToastNotificationManager::GetTemplateContent(ToastTemplateType::ToastImageAndText02).expect("get template failure");
        let header_node = xml.GetElementsByTagName(&("header".into())).unwrap();
        let header_node = header_node.Item(0).unwrap();
        let attr = header_node.Attributes().unwrap();
        let attr_title = attr.GetNamedItem(&("title".into())).unwrap();
        attr_title.SetInnerText(&(payload.title.into())).unwrap();

        let text_node = xml.GetElementsByTagName(&("text".into())).expect("get template failure");
        let first_line = text_node.GetAt(0).unwrap();
        first_line.AppendChild::<InParam<IXmlNode>,_>(InParam::owned(xml.CreateTextNode(&(payload.body.into())).unwrap().into())).expect("TODO: panic message");

        if let Some(img_url) = payload.image_url {
            let base64 = request_refer_image(&img_url, "", app).await.unwrap();
            let image_node = xml.GetElementsByTagName(&("image".into())).unwrap();
            let image_node = image_node.Item(0).unwrap();
            let img_attr = image_node.Attributes().unwrap();
            let img_src = img_attr.GetNamedItem(&("src".into())).unwrap();
            img_src.SetInnerText(&(base64.into())).unwrap();
        }
    }
    #[cfg(not(windows))]
    {
        let mut notify = tauri::api::notification::Notification::new(&app.config().tauri.bundle.identifier).title(payload.title).body(payload.body);
        if let Some(icon) = payload.icon {
            notify = notify.icon(icon);
        }
        if payload.has_sound {
            notify = notify.sound(Sound::Default)
        }
        notify.show()?;
    }

    Ok(())
}