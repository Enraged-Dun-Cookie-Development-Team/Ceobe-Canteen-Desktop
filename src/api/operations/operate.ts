import { copyInfo, openUrlInUserBrowser } from "../function";
import {
  appWindow,
  currentMonitor,
  getAll,
  PhysicalSize,
  WebviewWindow,
} from "@tauri-apps/api/window";

class Operate {
  async openNotificationWindow(cookie: any) {
    console.log(`send Notification`);

    let monitor = await currentMonitor();
    let size = monitor?.size;
    let window: WebviewWindow = getAll().find(
      (window: WebviewWindow) => window.label == "notification",
    );
    let winSize = await window.outerSize();
    let w = size?.width ?? 1920;
    let h = size?.height ?? 1080;
    await window.setPosition(
      new PhysicalSize(w - winSize.width, h - winSize.height),
    );
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
}

const operate = new Operate();

export default operate;
