import requestClient, { Payload } from "../../utils/requestUtil";
import { Response } from "@tauri-apps/api/http";

/**
 * 返回带有cookie_id和update_cookie_id的json
 * @param comb_id string
 * @returns {*}
 */
export function getCookieNewestInfo(
  comb_id: string,
): Promise<Response<Payload<any>>> {
  return requestClient.request({
    url: `http://cdn-dev.ceobecanteen.top/datasource-comb/${comb_id}`,
    method: "GET",
  });
}
