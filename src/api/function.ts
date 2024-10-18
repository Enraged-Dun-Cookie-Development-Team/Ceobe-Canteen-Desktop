import { readTextFile } from "@tauri-apps/plugin-fs";
import { open } from "@tauri-apps/plugin-shell";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
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
  console.log(data);
  if (data.type == "text") {
    await writeText(data.data);
  } else {
    await invoke("copy_image", { image: data.data });
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

export async function bootStartSetting(isBoot: boolean): Promise<boolean> {
  if (isBoot) {
    return await invoke<boolean>("set_auto_launch", { autoLaunch: true });
  } else {
    return await invoke<boolean>("set_auto_launch", { autoLaunch: false });
  }
}

export async function getBootStartSetting(): Promise<boolean> {
  return await invoke<boolean>("auto_launch_setting");
}
