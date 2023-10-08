import requestClient, { Payload, Response } from "../../utils/requestUtil";

export interface CombineId {
  /**
   * 数据源组合id
   */
  datasource_comb_id: string;
}

/**
 * 返回带有一个datasource_comb_id的json
 * @param uuids list[uuid]
 * @returns {*}
 */
export function getDatasourceComb(
  uuids: string[],
): Promise<Response<Payload<CombineId>>> {
  return requestClient.requestPayload({
    url: `/canteen/user/getDatasourceComb`,
    method: "POST",
    body: {
      datasource_push: uuids,
    },
  });
}
