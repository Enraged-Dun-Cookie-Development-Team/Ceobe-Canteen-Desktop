import {
  bootStartSetting,
  copyInfo,
  getBootStartSetting,
  openUrlInUserBrowser,
} from "../function";
import {
  appWindow,
  currentMonitor,
  getAll,
  LogicalPosition,
  PhysicalSize,
  WebviewWindow,
} from "@tauri-apps/api/window";
import { Cookie } from "../resourceFetcher/cookieList";

class Operate {
  async openNotificationWindow(cookie: Cookie) {
    console.log(`send Notification`);

    let monitor = await currentMonitor();
    let size = monitor?.size;
    let window: WebviewWindow = getAll().find(
      (window: WebviewWindow) => window.label == "notification",
    );
    let winSize = await window.outerSize();
    console.log(winSize, size);
    let w = size?.width ?? 1920;
    let h = size?.height ?? 1080;
    console.log(w, h);
    await window.setPosition(
      new LogicalPosition(w - winSize.width, h - winSize.height),
    );
    console.log(await window.outerPosition());
    await window.emit("info", cookie);
    await window.show();
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
  async exit() {}

  async bootSetting(isBoot: boolean) {
    return await bootStartSetting(isBoot);
  }

  async getBootSetting() {
    return await getBootStartSetting();
  }
}

const operate = new Operate();

export default operate;
