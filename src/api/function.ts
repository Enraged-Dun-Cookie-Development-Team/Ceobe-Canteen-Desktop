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
  console.log(data)
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

export async function bootStartSetting(isBoot: boolean) {
  if (isBoot) {
    await invoke("set_auto_launch", { auto_launch: true, hidden: true });
  } else {
    await invoke("set_auto_launch", { auto_launch: false, hidden: false });
  }
  return await getBootStartSetting();
}

export async function getBootStartSetting(): Promise<boolean> {
  return await invoke<boolean>("auto_launch_setting");
}
//
// // 是否开机自启
// ipcMain.handle("bootSetting", async (event, isBoot) => {
//   if (isBoot) {
//     //设置开机启动
//     app.setLoginItemSettings({
//       openAtLogin: true,
//       openAsHidden: true,
//     });
//   } else {
//     app.setLoginItemSettings({
//       openAtLogin: false,
//       openAsHidden: false,
//     });
//   }
//   //获取是否开机启动
//   const { openAtLogin } = app.getLoginItemSettings();
//   return openAtLogin;
// });
//
// ipcMain.handle("getBootSetting", async (event) => {
//   //获取是否开机启动
//   const { openAtLogin } = app.getLoginItemSettings();
//   return openAtLogin;
// });
//
// // 判断版本号大小
// ipcMain.handle("judgmentVersion", async (event, v1, v2) => {
//   console.log(v1);
//   console.log(v2);
//   if (v1 == v2) {
//     return false;
//   }
//
//   const vs1 = v1.split(".").map((a) => parseInt(a));
//   const vs2 = v2.split(".").map((a) => parseInt(a));
//
//   const digit = Math.min(vs1.length, vs2.length);
//   for (let i = 0; i < digit; i++) {
//     if (vs1[i] > vs2[i]) {
//       return true;
//     } else if (vs1[i] < vs2[i]) {
//       return false;
//     }
//   }
//
//   if (digit == vs1.length) {
//     return false;
//   } else {
//     return true;
//   }
// });
