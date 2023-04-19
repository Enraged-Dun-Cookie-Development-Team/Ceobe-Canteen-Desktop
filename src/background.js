import {app, protocol, BrowserWindow} from 'electron'
import {createProtocol} from 'vue-cli-plugin-electron-builder/lib'
import installExtension, {VUEJS3_DEVTOOLS} from 'electron-devtools-installer'
import path from "path";

const isDevelopment = process.env.NODE_ENV !== 'production'
// 这句话是让我可以操作iframe的内容
app.commandLine.appendSwitch("disable-site-isolation-trials");
require('./api/electronFunction.js')

// 必须在应用程序准备就绪之前注册协议
protocol.registerSchemesAsPrivileged([
    {scheme: 'app', privileges: {secure: true, standard: true}}
])

async function createWindow() {

    // 主页面窗口状态
    const win = new BrowserWindow({
        width: 1200,
        height: 600,
        // 这里碰到了大问题 不加载preload 解决方案 添加 nodeIntegration:true,
        // https://stackoverflow.com/questions/60814430/electron-builder-with-browserwindow-and-preload-js-unable-to-load-preload-scrip
        webPreferences: {
            nodeIntegration: true,
            preload: path.resolve(__dirname, 'preload.js'),
            webSecurity: false,
            webviewTag:true
        }
    })

    // 允许iframe访问第三方url
    win.webContents.session.webRequest.onHeadersReceived({urls: ["*://*/*"]},
        (d, c) => {
            if (d.responseHeaders['X-Frame-Options']) {
                delete d.responseHeaders['X-Frame-Options'];
            } else if (d.responseHeaders['x-frame-options']) {
                delete d.responseHeaders['x-frame-options'];
            }

            c({cancel: false, responseHeaders: d.responseHeaders});
        });


    if (process.env.WEBPACK_DEV_SERVER_URL) {
        // Load the url of the dev server if in development mode
        await win.loadURL(process.env.WEBPACK_DEV_SERVER_URL)
        if (!process.env.IS_TEST) win.webContents.openDevTools()
    } else {
        createProtocol('app')
        // Load the index.html when not in development
        win.loadURL('app://./index.html')
    }
}

// 当所有窗口都关闭时，退出应用程序。
app.on('window-all-closed', () => {
    if (process.platform !== 'darwin') {
        app.quit()
    }
})

app.on('activate', () => {
    // 在 macOS 上，当单击 dock 图标并且没有其他窗口打开时，通常会重新创建应用程序中的窗口。
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
})

// 当 Electron 完成初始化并准备好创建浏览器窗口时，将调用此方法。在此事件发生后，才能使用某些 API。
app.on('ready', async () => {
    if (isDevelopment && !process.env.IS_TEST) {
        // Install Vue Devtools
        console.log(process.versions.electron)
        console.log(process.versions.node);

        try {
            await installExtension(VUEJS3_DEVTOOLS)
        } catch (e) {
            console.error('Vue Devtools failed to install:', e.toString())
        }
    }
    createWindow()
})

// 在开发模式下，根据父进程的请求进行干净退出。
if (isDevelopment) {
    if (process.platform === 'win32') {
        process.on('message', (data) => {
            if (data === 'graceful-exit') {
                app.quit()
            }
        })
    } else {
        process.on('SIGTERM', () => {
            app.quit()
        })
    }
}
