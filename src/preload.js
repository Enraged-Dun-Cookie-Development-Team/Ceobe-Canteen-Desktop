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
    openWindow: (data) => ipcRenderer.invoke('newWindow', data),
})
