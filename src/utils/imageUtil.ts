export function getImage(url: string): string {
  return url;
}

// 根据base64获取图片信息
export function getImageFromBase64(base64String: string) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.onload = function () {
      resolve(this);
    };
    img.onerror = function () {
      reject("Error loading image");
    };
    img.src = base64String;
  });
}
