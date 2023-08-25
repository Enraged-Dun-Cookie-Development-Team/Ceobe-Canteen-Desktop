import { resolve, resolveResource } from "@tauri-apps/api/path";

export function getImage(url: string, isLocal = true): string {
  // url =url;
  // const path = await resolveResource(url);
  const ur = url;
  // const ur = new URL(url).href;
  console.log(import.meta);
  console.log(ur);
  return ur;
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
