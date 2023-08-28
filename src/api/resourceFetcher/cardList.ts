import requestClient, { Payload } from "../../utils/requestUtil";
import { Response } from "@tauri-apps/api/http";

export interface CardListItem {
  datasource: string;
  id: string;
  timeForSort: number;
  timeForDisplay: string;
  content: string;
  jumpUrl: string;
}

/**
 * 获取列表
 */
export function getCardList(): Promise<
  Response<Payload<Map<string, CardListItem>>>
> {
  return requestClient.requestPayload({
    url: `https://temp.ceobecanteen.top/canteen/cardList`,
    method: "GET",
  });
}
