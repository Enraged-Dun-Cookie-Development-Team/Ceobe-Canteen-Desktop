import {appWindow, getAll, WebviewWindow} from "@tauri-apps/api/window";
import {listen, UnlistenFn, Event} from "@tauri-apps/api/event";
import {Cookie} from "../resourceFetcher/cookieList";

class NotificationOperate {
    async getInfo(callback: (event: string, payload: Cookie) => void,): Promise<UnlistenFn> {
        return await listen<Cookie>("new_cookie_info", (event: Event<Cookie>) => callback(event.event, event.payload));
    }

    async closeWindow() {
        let window:WebviewWindow|null = getAll().find((window:WebviewWindow)=>window.label =="main")
        if (window){
            if (await window.isMinimized()){
                await window.unminimize()
            }
            if (!await window.isVisible()){
                await window.show()
            }
            await window.setFocus()
        }

        await appWindow.hide();
    }
}

const notification = new NotificationOperate();

export default notification;
