const {contextBridge, ipcRenderer, remote, shell} = require('electron');
const {getCardList, getAnnouncementInfo, getResourceInfo} = require("@/api/list");

contextBridge.exposeInMainWorld('versions', {
    node: () => process.versions.node, chrome: () => process.versions.chrome, electron: () => process.versions.electron,
})

contextBridge.exposeInMainWorld('ceobeRequest', {
    getCardList: () => getCardList(),
    getAnnouncementInfo: () => getAnnouncementInfo(),
    getResourceInfo: () => getResourceInfo(),
    getHasRefererImageBase64: (url) => ipcRenderer.invoke('getHasRefererImageBase64', url),
    getLocalFileText: (path) => ipcRenderer.invoke("getLocalFileText", path)
})

contextBridge.exposeInMainWorld('operate', {
    openNotificationWindow: (data) => {
        ipcRenderer.send('notification-close')
        ipcRenderer.invoke('openNotificationWindow', data)
    },
    openUrl: (url) => ipcRenderer.invoke('openUrl', url),
})

contextBridge.exposeInMainWorld('notification', {
    getInfo: (callback) => ipcRenderer.on('info', callback),
    closeWindow: () => ipcRenderer.send('notification-close')
});