import path from 'path';
import { createProtocol } from 'vue-cli-plugin-electron-builder/lib';
import { app, ipcMain, Menu, Tray } from 'electron';

const { BrowserWindow, screen } = require('electron');

let tray = null;

let win = null;

app.whenReady().then(() => {
  ipcMain.handle('openNotificationWindow', (event, data) => {
    createNotificationWindow(data);
  });
  // 托盘
  tray = new Tray('src/assets/image/logo/icon.png');
  tray.setToolTip('小刻食堂持续蹲饼中');
  // 菜单
  const contextMenu = Menu.buildFromTemplate([
    {
      label: '退出小刻食堂',
      click: () => {
        app.quit(); // 退出应用程序
      },
    },
  ]);
  tray.setContextMenu(contextMenu);

  if (process.platform !== 'darwin') {
    tray.on('click', () => {
      if (win && !win.isDestroyed()) {
        // 在窗口存在且未被销毁时打开窗口
        win.show();
      } else {
        // 在窗口不存在或已被销毁时创建新的窗口 （一般不会）
        createWindow().then();
      }
    });
  }
});

export async function createWindow() {
  // 主页面窗口状态
  win = new BrowserWindow({
    width: 1200,
    height: 600,
    useContentSize: true,
    frame: false,
    // 这里碰到了大问题 不加载preload 解决方案 添加 nodeIntegration:true,
    // https://stackoverflow.com/questions/60814430/electron-builder-with-browserwindow-and-preload-js-unable-to-load-preload-scrip
    webPreferences: {
      nodeIntegration: true,
      preload: path.resolve(__dirname, 'preload.js'),
      webSecurity: false,
      webviewTag: true,
    },
  });

  // 允许iframe访问第三方url
  win.webContents.session.webRequest.onHeadersReceived({ urls: ['*://*/*'] }, (d, c) => {
    if (d.responseHeaders['X-Frame-Options']) {
      delete d.responseHeaders['X-Frame-Options'];
    } else if (d.responseHeaders['x-frame-options']) {
      delete d.responseHeaders['x-frame-options'];
    }

    c({ cancel: false, responseHeaders: d.responseHeaders });
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    // Load the url of the dev server if in development mode
    await win.loadURL(process.env.WEBPACK_DEV_SERVER_URL);
    if (!process.env.IS_TEST) {
      win.webContents.openDevTools();
    }
  } else {
    createProtocol('app');
    // Load the index.html when not in development
    await win.loadURL('dist/index.html');
  }
}

let notificationWindow = null;

export async function createNotificationWindow(data = {}) {
  let { workAreaSize } = require('electron').screen.getPrimaryDisplay();
  notificationWindow = new BrowserWindow({
    width: 400,
    height: 355,
    frame: false,
    resizable: false,
    x: workAreaSize.width - 410,
    y: workAreaSize.height - 365,
    webPreferences: {
      webSecurity: false,
      nodeIntegration: true,
      preload: path.resolve(__dirname, 'preload.js'),
    },
    alwaysOnTop: true,
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    // Load the url of the dev server if in development mode
    await notificationWindow.loadURL(process.env.WEBPACK_DEV_SERVER_URL + 'notification');
    if (!process.env.IS_TEST) {
      notificationWindow.webContents.openDevTools();
    }
  } else {
    createProtocol('app');
    // Load the index.html when not in development
    await notificationWindow.loadFile('dist/index.html', {
      hash: '/notification',
    });
  }

  notificationWindow.webContents.on('did-finish-load', () => {
    notificationWindow.webContents.send('info', data);
  });
}

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
      preload: path.resolve(__dirname, 'preload.js'),
    },
  });

  if (process.env.WEBPACK_DEV_SERVER_URL) {
    // Load the url of the dev server if in development mode
    await backgroundWindow.loadURL(process.env.WEBPACK_DEV_SERVER_URL + 'background');
    if (!process.env.IS_TEST) {
      backgroundWindow.webContents.openDevTools();
    }
  } else {
    createProtocol('app');
    // Load the index.html when not in development
    await backgroundWindow.loadFile('dist/index.html', {
      hash: '/background',
    });
  }
}
ipcMain.on('newest-timeline', (_, cookies) => {
  sendWindowMessage(win, 'newest-timeline', cookies);
});
ipcMain.on('need-timeline', () => {
  sendWindowMessage(backgroundWindow, 'need-timeline');
});
ipcMain.on('update-datasource-comb', () => {
  sendWindowMessage(backgroundWindow, 'update-datasource-comb');
});

function sendWindowMessage(targetWindow, channel, args) {
  if (targetWindow) {
    targetWindow.webContents.send(channel, args);
  }
}

ipcMain.on('notification-close', () => {
  if (notificationWindow) {
    notificationWindow.close();
    notificationWindow = null;
  }
});

// 最小化等
ipcMain.handle('minus', (event) => {
  win.minimize();
});

// 窗口最大化
ipcMain.handle('maximize', (event) => {
  if (win.isMaximized()) {
    win.restore();
  } else {
    win.maximize();
  }
});

// 窗口关闭
ipcMain.handle('close', (event) => {
  win.hide();
});

// 退出程序
ipcMain.handle('exit', (event) => {
  win.hide();
});

// 在窗口关闭时，销毁托盘图标
app.on('window-all-closed', () => {
  if (tray) {
    tray.destroy();
  }
});
