import { readTextFile } from "@tauri-apps/api/fs";
import { open } from "@tauri-apps/api/shell";
import { writeText } from "@tauri-apps/api/clipboard";
import { invoke } from "@tauri-apps/api";

// 获取文件
export async function getLocalFileText(path: string): Promise<string> {
  return await readTextFile(path);
}

// 使用电脑默认浏览器打开链接
export async function openUrlInUserBrowser(url: string) {
  await open(url);
}

export async function copyInfo(data: { data: string; type: string }) {
  if (data.type == "text") {
    await writeText(data.data);
  } else {
    throw Error(`type:${data.type} not support yet`);
  }
}

export async function getHasRefererImageBase64(
  url: string,
  referer: string = "https://weibo.com/",
): Promise<string> {
  console.log(url);
  console.log(referer);
  return await invoke<string>("request_refer_image", {
    url: url,
    refer: referer,
  });
}
