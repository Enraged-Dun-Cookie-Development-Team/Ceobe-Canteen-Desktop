import {
  createNotificationWindow,
  createWindow,
  createBackgroundWindow,
  backgroundWindow,
  win,
} from "@/api/window";

// 当所有窗口都关闭时，退出应用程序。
app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  // 在 macOS 上，当单击 dock 图标并且没有其他窗口打开时，通常会重新创建应用程序中的窗口。
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});

const gotTheLock = app.requestSingleInstanceLock();

if (!gotTheLock) {
  app.quit();
} else {
  app.on("second-instance", (event, commandLine, workingDirectory) => {
    //有人试图运行第二个实例，我们应该关注我们的窗口
    if (win) {
      if (win.isMinimized()) win.restore();
      win.focus();
      win.show();
    }
  });
  // 当 Electron 完成初始化并准备好创建浏览器窗口时，将调用此方法。在此事件发生后，才能使用某些 API。
  app.on("ready", async () => {
    if (isDevelopment && !process.env.IS_TEST) {
      // Install Vue Devtools
      console.log(process.versions.electron);
      console.log(process.versions.node);

      try {
        await installExtension(VUEJS3_DEVTOOLS);
      } catch (e) {
        console.error("Vue Devtools failed to install:", e.toString());
      }
    }
    await createWindow();
    await createBackgroundWindow();
    // await createNotificationWindow()
  });
}

function sendWindowMessage(targetWindow, message, payload) {
  if (typeof targetWindow === "undefined") {
    console.log("Target window does not exist");
    return;
  }
  targetWindow.webContents.send(message, payload);
}
