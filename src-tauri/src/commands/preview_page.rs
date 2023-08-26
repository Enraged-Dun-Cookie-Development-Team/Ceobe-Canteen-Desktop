use reqwest::Url;
use std::thread::sleep;
use std::time::Duration;
use tauri::http::{Request, Response};
use tauri::{command, AppHandle, Manager, WindowBuilder, WindowEvent, WindowUrl};
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
#[instrument(skip_all,err,name="LeavePreview")]
pub fn back_preview(app:AppHandle)->tauri::Result<()>{
    if let Some(win) = app.get_window(WINDOWS_NAME) {
        win.hide()?;
    }
    Ok(())
}

#[command(async)]
#[instrument(skip(app),err,name="PreviewPage")]
pub async fn read_detail(app: AppHandle, url: Url, title: String) -> tauri::Result<()> {
    let main  = app.get_window("main").unwrap();
    let window = if let Some(window) = app.get_window(WINDOWS_NAME) {
        info!(state="WindowExist",action="DirectUsing");
        window.eval(&format!("window.location.replace('{url}')"))?;
        sleep(Duration::from_millis(500));

        window
    } else {
        info!(state="WindowNotExist",action="CreateWindow");
        let w = WindowBuilder::new(&app, WINDOWS_NAME, WindowUrl::External(url))
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
                win.hide().ok();
                let _ = tmp_main.emit("close-main", ());
            }
            if let WindowEvent::CloseRequested { api, .. } = ev {
                api.prevent_close();
                win.hide().ok();
            }
        });
        // wait window open
        sleep(Duration::from_millis(500));
        #[cfg(windows)]
        {
            const LEFT_W:i32 = 504i32;
            const TOP_H:i32 = 124i32;
            let size = main.inner_size()?;
            let width = size.width as i32 - LEFT_W;
            let height = size.height as i32 - TOP_H;

            use windows::{
                Win32::UI::WindowsAndMessaging::MoveWindow,
                Win32::UI::WindowsAndMessaging::SetParent
            };
            let new_hwnd =w.hwnd()?;
            let main_hwnd = main.hwnd()?;
            unsafe {
                SetParent(new_hwnd, main_hwnd);
                MoveWindow(new_hwnd, LEFT_W, TOP_H, width, height, true);
            }
            main.on_window_event(move|event|{
                if let WindowEvent::Resized(size) = event {
                    let width = size.width as i32 - LEFT_W;
                    let height = size.height as i32 - TOP_H;
                    unsafe { MoveWindow(new_hwnd, LEFT_W, TOP_H, width, height, true); }
                }
            })
        }
        w
    };
    window.eval(include_str!("init_script.js"))?;
    window.set_title(&title)?;

    info!(state="Ready",action="WindowReadyToShow");
    window.set_focus()?;
    window.show()?;
    Ok(())
}

fn handle_inject_css(_: &Request, resp: &mut Response) {
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

        *resp.body_mut() = resp_body;
    }
}
