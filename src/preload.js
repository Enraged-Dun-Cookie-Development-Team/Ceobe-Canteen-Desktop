const {contextBridge, ipcRenderer, clipboard} = require('electron');
const {getCardList} = require("@/api/list");

contextBridge.exposeInMainWorld('versions', {
    node: () => process.versions.node,
    chrome: () => process.versions.chrome,
    electron: () => process.versions.electron,
})

contextBridge.exposeInMainWorld('ceobeRequest', {
    getCardList1: () => getCardList,
})

contextBridge.exposeInMainWorld('operate', {
    openWindow: (data) => ipcRenderer.invoke('newWindow', data),
})