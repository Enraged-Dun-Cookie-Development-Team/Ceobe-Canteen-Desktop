import { copyInfo, openUrlInUserBrowser } from "../function";
import { appWindow } from "@tauri-apps/api/window";

class Operate {
  openNotificationWindow(cookie: any) {}

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
    await appWindow.close();
  }
}

const operate = new Operate();

export default operate;
