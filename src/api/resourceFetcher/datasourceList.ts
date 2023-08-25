import requestClient, { Payload } from "../../utils/requestUtil";
import { Response } from "@tauri-apps/api/http";

export interface DatasourceItem {
  /**
   * 数据源头像
   */
  avatar: string;
  /**
   * 数据源类型
   */
  datasource: string;
  /**
   * 数据源相关唯一id
   */
  db_unique_key: string;
  /**
   * 跳转url（null就是没办法跳转）
   */
  jump_url: null | string;
  /**
   * 数据源昵称
   */
  nickname: string;
  /**
   * 平台类型，具体显示名字让ui或者产品定
   * - bilibili: b站
   * - weibo: 微博
   * - netease-cloud-music: 网易云音乐
   * - arknights-game: 方舟-游戏内
   * - arknights-game: 方舟-网站
   */
  platform: string;
  /**
   * 数据源唯一标识（用于前后端交互标识）
   */
  unique_id: string;
}
/**
 * 返回带uuid的资源列表
 * @returns {*}
 */
export function getConfigDatasourceList(): Promise<
  Response<Payload<DatasourceItem[]>>
> {
  return requestClient.request({
    url: `/canteen/config/datasource/list`,
    method: "GET",
  });
}

export const getResourceList = getConfigDatasourceList;
