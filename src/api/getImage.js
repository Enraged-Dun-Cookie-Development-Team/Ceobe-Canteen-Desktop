const {app, ipcMain} = require('electron')
app.whenReady().then(() => {
    const {net} = require('electron');
    const getWeiboImageBase64 = (url) => {
        return new Promise((resolve, reject) => {
            const request = net.request(url);
            request.setHeader('Referer', 'https://weibo.com/');
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
    ipcMain.handle('getWeiboImageBase64', (event, url) => {
        return getWeiboImageBase64(url);
    })
})

