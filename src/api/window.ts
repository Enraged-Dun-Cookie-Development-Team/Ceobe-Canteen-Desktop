import path from "path";
import { createProtocol } from "vue-cli-plugin-electron-builder/lib";

const { BrowserWindow, screen } = require("electron");

export let backgroundWindow = null;

/*  创建时间轴后台窗口
    在后台调用queryTimeline接口
    轮询时间轴数据
*/
export async function createBackgroundWindow() {
  backgroundWindow = new BrowserWindow({
    show: false,
    webPreferences: {
      nodeIntegration: true,
      webSecurity: false,
      preload: path.resolve(__dirname, "preload.js"),
    },
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    // Load the url of the dev server if in development mode
    await backgroundWindow.loadURL(
      process.env.WEBPACK_DEV_SERVER_URL + "#/background",
    );
    if (!process.env.IS_TEST) {
      backgroundWindow.webContents.openDevTools();
    }
  } else {
    createProtocol("app");
    // Load the index.html when not in development
    await backgroundWindow.loadFile("index.html", {
      hash: "/background",
    });
  }
}

ipcMain.on("newest-timeline", (_, cookies) => {
  sendWindowMessage(win, "newest-timeline", cookies);
});
ipcMain.on("need-timeline", () => {
  sendWindowMessage(backgroundWindow, "need-timeline");
});
ipcMain.on("update-datasource-comb", () => {
  sendWindowMessage(backgroundWindow, "update-datasource-comb");
});
ipcMain.on("search-word", (_, searchWord) => {
  sendWindowMessage(win, "search-word", searchWord);
});

function sendWindowMessage(targetWindow, channel, args) {
  if (targetWindow) {
    targetWindow.webContents.send(channel, args);
  }
}

ipcMain.on("notification-close", () => {
  if (notificationWindow) {
    notificationWindow.close();
    notificationWindow = null;
  }
});
