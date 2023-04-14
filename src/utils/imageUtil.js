export function getImage(url, isLocal = true) {
    if (url) {
        return new URL(isLocal ? '..' + url : url, import.meta.url).href;
    } else {
        return null
    }
}

// 根据base64获取图片信息
export function getImageFromBase64(base64String) {
    return new Promise((resolve, reject) => {
        const img = new Image();
        img.onload = function () {
            resolve(this);
        };
        img.onerror = function () {
            reject('Error loading image');
        };
        img.src = base64String;
    });
}
