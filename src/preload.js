const { contextBridge, ipcRenderer, remote, shell } = require('electron');
const {getCardList, getAnnouncementInfo, getResourceInfo} = require("@/api/list");

contextBridge.exposeInMainWorld('versions', {
    node: () => process.versions.node, chrome: () => process.versions.chrome, electron: () => process.versions.electron,
})

contextBridge.exposeInMainWorld('ceobeRequest', {
    getCardList: () => getCardList(),
    getAnnouncementInfo: () => getAnnouncementInfo(),
    getResourceInfo: () => getResourceInfo(),
    getWeiboImageBase64: (url) => ipcRenderer.invoke('getWeiboImageBase64', url)
})

contextBridge.exposeInMainWorld('operate', {
    openWindow: (data) => ipcRenderer.invoke('newWindow', data),
})
