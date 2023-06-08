import {app, protocol,  BrowserWindow} from 'electron'
import installExtension, {VUEJS3_DEVTOOLS} from 'electron-devtools-installer'
import {createNotificationWindow, createWindow} from "@/api/window";

const isDevelopment = process.env.NODE_ENV !== 'production'
// 这句话是让我可以操作iframe的内容
app.commandLine.appendSwitch("disable-site-isolation-trials");
require('./api/function.js')
require('./api/window.js')


// 必须在应用程序准备就绪之前注册协议
protocol.registerSchemesAsPrivileged([
    {scheme: 'app', privileges: {secure: true, standard: true}}
])

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
    await createWindow()
    // await createNotificationWindow()
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
