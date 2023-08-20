const { contextBridge, ipcRenderer, remote, shell } = require('electron');
const {
  getCardList,
  getAnnouncementInfo,
  getResourceInfo,
  getResourceList,
  getDatasourceComb,
  getDatasourceCombList,
  getCookieList,
} = require('@/api/list');

contextBridge.exposeInMainWorld('versions', {
  node: () => process.versions.node,
  chrome: () => process.versions.chrome,
  electron: () => process.versions.electron,
});

contextBridge.exposeInMainWorld('ceobeRequest', {
  getCardList: () => getCardList(),
  getResourceList: () => getResourceList(),
  getDatasourceComb: (uuids) => getDatasourceComb(uuids),
  getDatasourceCombList: (comb_id) => getDatasourceCombList(comb_id),
  getCookieList: (comb_id, cookie_id, update_cookie_id) => getCookieList(comb_id, cookie_id, update_cookie_id),
  getAnnouncementInfo: () => getAnnouncementInfo(),
  getResourceInfo: () => getResourceInfo(),
  getHasRefererImageBase64: (url) => ipcRenderer.invoke('getHasRefererImageBase64', url),
  getLocalFileText: (path) => ipcRenderer.invoke('getLocalFileText', path),
});

contextBridge.exposeInMainWorld('operate', {
  openNotificationWindow: (data) => {
    ipcRenderer.send('notification-close');
    ipcRenderer.invoke('openNotificationWindow', data);
  },
  openUrlInBrowser: (url) => ipcRenderer.invoke('openUrlInBrowser', url),
  copy: (data) => ipcRenderer.invoke('copy', data),
  minus: () => ipcRenderer.invoke('minus'),
  maximize: () => ipcRenderer.invoke('maximize'),
  close: () => ipcRenderer.invoke('close'),
  exit: () => ipcRenderer.invoke('exit'),
  bootSetting:(isBoot) => ipcRenderer.invoke('bootSetting',isBoot),
});

contextBridge.exposeInMainWorld('notification', {
  getInfo: (callback) => ipcRenderer.on('info', callback),
  closeWindow: () => ipcRenderer.send('notification-close'),
});

contextBridge.exposeInMainWorld('newestTimeline', {
  getTimeline: (callback) => ipcRenderer.on('newest-timeline', callback),
  sendTimeline: (data) => ipcRenderer.send('newest-timeline', data),
  knowNeedTimeline: (callback) => ipcRenderer.on('need-timeline', callback),
  needTimeline: () => ipcRenderer.send('need-timeline'),
});

contextBridge.exposeInMainWorld('datasourceConfig', {
  datasourceCombUpdated: (callback) => ipcRenderer.on('update-datasource-comb', callback),
  updateDatasourceComb: () => ipcRenderer.send('update-datasource-comb'),
});

contextBridge.exposeInMainWorld('searchWordEvent', {
  getSearchWord: (callback) => ipcRenderer.on('search-word', callback),
  sendSearchWord: (data) => ipcRenderer.send('search-word', data),
});
