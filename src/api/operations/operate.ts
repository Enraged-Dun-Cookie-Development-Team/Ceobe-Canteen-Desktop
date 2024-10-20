import { invoke } from "@tauri-apps/api/core";
import {
  getCurrentWebviewWindow,
} from "@tauri-apps/api/webviewWindow";

import {
  bootStartSetting,
  copyInfo,
  getBootStartSetting,
  openUrlInUserBrowser,
} from "../function";

import { getAllWindows, Window } from "@tauri-apps/api/window";

const appWindow = getCurrentWebviewWindow();

class Operate {
  async getWindow(label:string):Promise<Window | null>{
    const allWindows = await getAllWindows()
    const window = allWindows.find((window:Window)=>window.label === label)
    if(window){
      return window
    }else{
      return null
    }
  }


  async copy(param: { data: string; type: string }) {
    await copyInfo(param);
  }

  async openUrlInBrowser(url: string) {
    await openUrlInUserBrowser(url);
  }

  async minus() {
    const window = appWindow;
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
