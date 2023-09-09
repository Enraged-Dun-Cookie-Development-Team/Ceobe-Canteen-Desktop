import {
  bootStartSetting,
  copyInfo,
  getBootStartSetting,
  openUrlInUserBrowser,
} from "../function";
import {
  appWindow,
  getAll,
  PhysicalPosition, PhysicalSize,
  WebviewWindow,
} from "@tauri-apps/api/window";
import { Cookie } from "../resourceFetcher/cookieList";
import {invoke} from "@tauri-apps/api";

class Operate {
  async openNotificationWindow(cookie: Cookie) {
    console.log(`send Notification`);

    let monitorInfo = await invoke<{
      work_space:PhysicalSize,
      left_top :PhysicalPosition
    }>("get_monitor_info");

    let size = monitorInfo.work_space;
    let window: WebviewWindow = getAll().find(
      (window: WebviewWindow) => window.label == "notification",
    );
    let winSize = await window.outerSize();
    console.log(winSize, size);
    let w = size?.width ?? 1920;
    let h = size?.height ?? 1080;
    console.log(w, h);
    await window.setPosition(
      new PhysicalPosition(
          w - winSize.width + monitorInfo.left_top.x,
          h - winSize.height + monitorInfo.left_top.y),
    );
    console.log(await window.outerPosition());
    console.log("send cookie " , cookie)
    await window.emit("new_cookie_info", cookie);
  }

  async copy(param: { data: string; type: string }) {
    await copyInfo(param);
  }

  async openUrlInBrowser(url: string) {
    await openUrlInUserBrowser(url);
  }

  async minus() {
    let window = appWindow;
    await window.minimize();
  }

  async maximize() {
    await appWindow.toggleMaximize();
  }

  async close() {
    await appWindow.hide();
  }
  async exit() {
    await invoke("quit")
  }

  async bootSetting(isBoot: boolean) {
    return await bootStartSetting(isBoot);
  }

  async getBootSetting() {
    return await getBootStartSetting();
  }

  async messageBeep(){
    await invoke("message_beep")
  }

  async hideNotifyIcon(){
    await invoke("hide_notification")
  }
}

const operate = new Operate();

export default operate;
