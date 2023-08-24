import { createWindow, createBackgroundWindow, win } from "@/api/window";

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
