import requestClient, { Payload } from "../../utils/requestUtil";
import { Response } from "@tauri-apps/api/http";

export interface DesktopVersion {
  /**
   * 百度云下载路径
   */
  baidu: string;
  /**
   * 百度云描述
   */
  baidu_text: string;
  /**
   * 描述
   */
  description: string;
  /**
   * dmg下载路径
   */
  dmg: string;
  /**
   * exe下载路径
   */
  exe: string;
  /**
   * 是否强制更新
   */
  force: boolean;
  /**
   * 上次强制更新版本
   */
  last_force_version: string;
  /**
   * dmg备用下载路径
   */
  spare_dmg: string;
  /**
   * exe备用下载路径
   */
  spare_exe: string;
  /**
   * 版本
   */
  version: string;
}

export function getVersion(
  version?: string,
): Promise<Response<Payload<DesktopVersion>>> {
  return requestClient.request({
    url: "/canteen/operate/version/desktop",
    method: "GET",
    query: {
      version: version,
    },
  });
}