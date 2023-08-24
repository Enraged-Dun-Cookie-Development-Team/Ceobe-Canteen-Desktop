use std::thread::sleep;
use std::time::Duration;
use reqwest::{ Url};
use tauri::{AppHandle, command, Manager, WindowBuilder, WindowEvent, WindowUrl};
use tauri::http::{Request, Response};

const WINDOWS_NAME: &str = "Preview";

const INSERT :&str = r#"<style type="text/css">
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

#[command(async)]
pub async fn init_preview(app: AppHandle, url: Url, title: String) -> tauri::Result<()> {
    let window = if let Some(window) = app.get_window(WINDOWS_NAME) {
        window.eval(&format!("window.location.replace('{url}')"))?;
        window.show()?;
        window
    } else {
        let w = WindowBuilder::new(&app, WINDOWS_NAME, WindowUrl::External(url))
            .title("Preview")
            .center()
            .inner_size(816f64, 648f64)
            .on_web_resource_request(handle_inject_css)
            .build()?;
        let win = w.clone();
        w.on_window_event(move |ev| match ev {
            WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                win.hide().ok();
            }
            _ => {}
        });
        // wait window open
        w
    };
    sleep(Duration::from_millis(500));
    window.eval(include_str!("init_script.js"))?;
    window.set_focus()?;
    window.set_title(&title)?;
    Ok(())
}



fn handle_inject_css(_:&Request,resp:&mut Response){

    if let Some(true) | None = resp.headers().get("content-type").and_then(|v|v.to_str().ok()).map(|s|s.starts_with("text/html")) {
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