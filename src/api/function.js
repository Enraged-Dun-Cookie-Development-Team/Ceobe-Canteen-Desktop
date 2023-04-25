const {app, ipcMain, shell, clipboard,nativeImage} = require('electron')
const fs = require("fs")

app.whenReady().then(() => {
    const {net} = require('electron');

    // 获取base64的图片
    const getHasRefererImageBase64 = (url, referer = "https://weibo.com/") => {
        return new Promise((resolve, reject) => {
            const request = net.request(url);
            request.setHeader('Referer', referer);
            request.on('response', (response) => {
                let chunks = [];
                response.on('data', (chunk) => {
                    chunks.push(chunk);
                });
                response.on('end', () => {
                    const buffer = Buffer.concat(chunks);
                    const base64 = buffer.toString('base64');
                    resolve(base64);
                });
            });
            request.on('error', (error) => {
                reject(error);
            });
            request.end();
        });
    };
    ipcMain.handle('getHasRefererImageBase64', (event, url, referer) => {
        return getHasRefererImageBase64(url, referer);
    })

    // 获取文件
    const getLocalFileText = (path) => {
        return new Promise((resolve, reject) => {
            fs.readFile(path, {encoding: "utf-8"}, (err, data) => {
                console.log(err)
                console.log(data)
                if (err) {
                    reject(err)
                }
                resolve(data)
            })
        })
    }
    ipcMain.handle('getLocalFileText', function (event, path) {
        return getLocalFileText(path)
    });

    // 使用电脑默认浏览器打开链接
    ipcMain.handle('openUrlInBrowser', (event, url) => {
        shell.openExternal(url);
    });

    ipcMain.handle('copy', (event, data) => {
        if (data.type == 'img') {
           let image = nativeImage.createFromDataURL(data.data)
            clipboard.writeImage(image)
        } else if (data.type == 'text') {
            clipboard.writeText(data.data)
        }
    })
})

