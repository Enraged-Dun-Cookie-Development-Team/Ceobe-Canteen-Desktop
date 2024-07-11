import requestClient, { Response } from "@/utils/requestUtil";

export interface NewestCookies {
  /**
   * 最新饼id
   */
  cookie_id: null | string;
  /**
   * 最新更新饼id
   */
  update_cookie_id: null | string;
}

/**
 * 返回带有cookie_id和update_cookie_id的json
 * @param comb_id string
 * @returns {*}
 */
export function getCookieNewestInfo(
  comb_id: string,
): Promise<Response<NewestCookies>> {
  return requestClient.request({
    requestTarget: "CDN",
    url: `/datasource-comb/${comb_id}`,
    method: "GET",
    headers: {
      "Cache-Control": "no-cache",
    },
  });
}
