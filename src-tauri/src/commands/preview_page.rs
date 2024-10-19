use reqwest::Url;
use std::borrow::Cow;
use std::thread::sleep;
use std::time::Duration;
use tauri::http::{Request, Response};
use tauri::{
    command, AppHandle, Emitter, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Window,
    WindowBuilder, WindowEvent,
};
use tracing::{info, instrument};

const WINDOWS_NAME: &str = "Preview";

const INSERT: &str = r#"<style type="text/css">
#bili-header-container,
#internationalHeader,
.van-popover,
.international-header,
.login-tip,
.bili-dyn-card-link-common,
.bili-dyn-item__footer,
.bili-dyn-item__panel{
  display:none!important
}
[class^="Frame_top_"],
[class^="Frame_side_"],
[class^="Bar_main_"],
[class^="title_wrap_"],
.woo-panel-main>footer,
[class^="Detail_box_"],
[class^="Main_side_"]{
    display:none!important
}
</style>
"#;

#[command]
#[instrument(skip_all, err, name = "LeavePreview")]
pub fn back_preview(app: AppHandle) -> tauri::Result<()> {
    if let Some(win) = app.get_webview_window(WINDOWS_NAME) {
        win.eval("window.location.replace('about:black')")?;
        win.hide()?;
    }
    Ok(())
}

#[command(async)]
#[instrument(skip(app, url), err, name = "PreviewPage")]
pub async fn read_detail(app: AppHandle, url: Url, title: String) -> tauri::Result<()> {
    let main = app.get_webview_window("main").unwrap();
    let window = if let Some(window) = app.get_webview_window(WINDOWS_NAME) {
        info!(state = "WindowExist", action = "DirectUsing");
        window.eval(&format!("window.location.replace('{url}')"))?;
        sleep(Duration::from_millis(500));

        window
    } else {
        info!(state = "WindowNotExist", action = "CreateWindow");
        let w = WebviewWindowBuilder::new(&app, WINDOWS_NAME, WebviewUrl::External(url))
            .title("Preview")
            .decorations(false)
            .visible(false)
            .resizable(false)
            .center()
            .inner_size(816f64, 648f64)
            .on_web_resource_request(handle_inject_css)
            .build()?;
        let win = w.clone();
        let tmp_main = main.clone();
        w.on_window_event(move |ev| {
            if let WindowEvent::Focused(false) = ev {
                if let Ok(true) = win.is_visible() {
                    win.hide().ok();
                    let _ = tmp_main.emit("close-main", ());
                }
            }
            if let WindowEvent::CloseRequested { api, .. } = ev {
                api.prevent_close();
                // win.hide().ok();
            }
        });
        // wait window open
        sleep(Duration::from_millis(500));
        move_window_to_fit(main, w.clone())?;
        w
    };
    window.eval(include_str!("init_script.js"))?;
    window.set_title(&title)?;

    info!(state = "Ready", action = "WindowReadyToShow");
    window.set_focus()?;
    window.show()?;
    Ok(())
}

fn handle_inject_css(_: Request<Vec<u8>>, resp: &mut Response<Cow<'static, [u8]>>) {
    if let Some(true) | None = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.starts_with("text/html"))
    {
        println!("inject");
        let payload = String::from_utf8_lossy(resp.body());
        let patten = regex::Regex::new(r#"<html [^>]+?><head>"#).expect("Bad regex");

        let ret = patten.find(&payload).expect("not found element");
        let insert = ret.end();
        let mut resp_body = Vec::with_capacity(resp.body().len() + INSERT.as_bytes().len());
        let origin = resp.body();
        resp_body.extend(&origin[0..insert]);
        resp_body.extend(INSERT.as_bytes());
        resp_body.extend(&origin[insert..]);

        *resp.body_mut() = resp_body.into();
    }
}
/// 将子窗口移动到目标位置中
fn move_window_to_fit(
    main_window: WebviewWindow,
    preview_window: WebviewWindow,
) -> tauri::Result<()> {
    const LEFT_W: i32 = 500i32;
    const TOP_H: i32 = 124i32;

    // let main_monitor = main_window.current_monitor()?;
    // let scale = if let Some(monitor) = main_monitor {
    //     monitor.scale_factor()
    // } else {
    //     1.0
    // };
    //
    // info!(monitor.scale = scale);
    //
    // let main_window_size = main_window.inner_size()?;
    // fn child_window_location(size: tauri::PhysicalSize<u32>, scale: f64) -> (i32, i32) {
    //     let width = size.width as i32 - (LEFT_W as f64 * scale) as i32;
    //     let height = size.height as i32 - (TOP_H as f64 * scale) as i32;
    //     (width, height)
    // }
    // let (w, h) = child_window_location(main_window_size, scale);
    // #[cfg(windows)]
    // {
    //     let new_hwnd = preview_window.hwnd()?;
    //     let main_hwnd = main_window.hwnd()?;
    //     unsafe {
    //         use windows::Win32::UI::WindowsAndMessaging::{MoveWindow, SetParent};
    //         SetParent(new_hwnd, main_hwnd);
    //         MoveWindow(
    //             new_hwnd,
    //             (LEFT_W as f64 * scale) as i32,
    //             (TOP_H as f64 * scale) as i32,
    //             w,
    //             h,
    //             true,
    //         );
    //     }
    // }
    // #[cfg(not(windows))]
    // {
    //     preview_window.set_size(tauri::PhysicalSize::new(w, h))?;
    //     let main_location = main_window.inner_position()?.to_logical::<u32>(scale);
    //     preview_window.set_position(tauri::LogicalPosition::new(
    //         main_location.x + LEFT_W as u32,
    //         main_location.y + TOP_H as u32,
    //     ))?;
    // }
    // main_window.on_window_event({
    //     #[cfg(windows)]
    //     let window = preview_window.hwnd()?;
    //     #[cfg(not(windows))]
    //     let window = preview_window.clone();
    //     move |event| {
    //         if let WindowEvent::Resized(size) = event {
    //             let (w, h) = child_window_location(*size, scale);
    //             #[cfg(windows)]
    //             {
    //                 use windows::Win32::UI::WindowsAndMessaging::MoveWindow;
    //                 unsafe {
    //                     MoveWindow(
    //                         window,
    //                         (LEFT_W as f64 * scale) as i32,
    //                         (TOP_H as f64 * scale) as i32,
    //                         w,
    //                         h,
    //                         true,
    //                     );
    //                 }
    //             }
    //             #[cfg(not(windows))]
    //             {
    //                 window
    //                     .set_size(tauri::PhysicalSize::new(w, h))
    //                     .expect("Cannot resize window");
    //             }
    //         } else if let WindowEvent::Moved(_pos) = event {
    //             #[cfg(not(windows))]
    //             {
    //                 let main_location = _pos;
    //                 use tauri::PhysicalPosition;
    //                 preview_window
    //                     .set_position(PhysicalPosition::new(
    //                         main_location.x + ((LEFT_W as f64 * scale) as i32),
    //                         main_location.y + ((LEFT_W as f64 * scale) as i32),
    //                     ))
    //                     .expect("cannot Move With Main");
    //             }
    //         }
    //     }
    // });

    Ok(())
}
