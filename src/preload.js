const {contextBridge, ipcRenderer } = require('electron');
const {getCardList} = require("@/api/list");

contextBridge.exposeInMainWorld('versions', {
    node: () => process.versions.node,
    chrome: () => process.versions.chrome,
    electron: () => process.versions.electron,
})

contextBridge.exposeInMainWorld('ceobeRequest', {
    getCardList: () => getCardList(),
})

contextBridge.exposeInMainWorld('operate', {
    openWindow: (data) => ipcRenderer.invoke('newWindow', data),
})