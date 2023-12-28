import requestClient, {Payload, Response} from "@/utils/requestUtil.js";


export interface Datum {
    /**
     * 头像
     */
    avatar: string;
    /**
     * 跳转链接
     */
    jump_url: string;
    /**
     * 名字
     */
    nickname: string;
}

export function getToolsLinks(): Promise<Response<Payload<Datum[]>>> {
    return requestClient.requestPayload({
        url: "/canteen/operate/toolLink/list", method: "GET"

    });
}