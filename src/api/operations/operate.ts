import { invoke } from "@tauri-apps/api/core";
import {
  getAllWindows,
  getCurrentWindow,
  Window,
} from "@tauri-apps/api/window";

import {
  bootStartSetting,
  copyInfo,
  getBootStartSetting,
  openUrlInUserBrowser,
} from "../function";

const appWindow = getCurrentWindow();

class Operate {
  async getWindow(label: string): Promise<Window | null> {
    const allWindows = await getAllWindows();
    const window = allWindows.find((window: Window) => window.label === label);

    return window ?? null;
  }

  async copy(param: { data: string; type: string }) {
    await copyInfo(param);
  }

  async openUrlInBrowser(url: string) {
    await openUrlInUserBrowser(url);
  }

  async minus() {
    const window = getCurrentWindow();
    await window.minimize();
  }

  async maximize() {
    await appWindow.toggleMaximize();
  }

  async close() {
    await appWindow.hide();
  }

  async exit() {
    await invoke("quit");
  }

  async bootSetting(isBoot: boolean) {
    return await bootStartSetting(isBoot);
  }

  async getBootSetting() {
    return await getBootStartSetting();
  }
}

const operate = new Operate();

export default operate;
