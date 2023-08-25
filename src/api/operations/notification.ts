import { appWindow } from "@tauri-apps/api/window";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { Cookie } from "../resourceFetcher/cookieList";

class NotificationOperate {
  async getInfo(
    callback: (event: string, payload: Cookie) => void,
  ): Promise<UnlistenFn> {
    return await listen<Cookie>("info", callback);
  }

  async closeWindow() {
    await appWindow.hide();
  }
}
const notification = new NotificationOperate();

export default notification;
