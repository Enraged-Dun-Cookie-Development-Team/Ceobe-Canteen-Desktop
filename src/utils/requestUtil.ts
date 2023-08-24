import { Client, getClient, HttpOptions, Response } from "@tauri-apps/api/http";

const baseUrl = "http://server-dev.ceobecanteen.top/api/v1";

const showStatus = (status: number) => {
  let message: string;
  switch (status) {
    case 400:
      message = "请求错误(400)";
      break;
    case 401:
      message = "未授权，请重新登录(401)";
      break;
    case 402:
      message = "拒绝访问(402)";
      break;
    case 404:
      message = "请求出错(404)";
      break;
    case 408:
      message = "请求超时(408)";
      break;
    case 500:
      message = "服务器错误(500)";
      break;
    case 501:
      message = "服务未实现(501)";
      break;
    case 502:
      message = "网络错误(502)";
      break;
    case 503:
      message = "服务不可用(503)";
      break;
    case 504:
      message = "网络超时(504)";
      break;
    case 505:
      message = "HTTP版本不受支持(505)";
      break;
    default:
      message = `连接出错(${status})!`;
  }
  return `${message}，请检查网络或联系管理员！`;
};

class RequestClient {
  client?: Client;
  waitInit: () => void;

  constructor() {
    this.waitInit = () =>
      new Promise<void>((resolve) => {
        getClient({ connectTimeout: 3 }).then((client: Client) => {
          console.log("client Ready");
          this.client = client;
          resolve();
        });
      });
  }

  async request<T>(options: HttpOptions): Response<Payload<T>> {
    await this.waitInit();
    const client = this.client;

    options.headers = {
      "Content-Type": "application/json;charset=utf-8",
      "Cache-Control": "no-cache",
    };
    if (!options.url.startsWith("http")) {
      options.url = `${baseUrl}${options.url}`;
    }

    try {
      console.log(`sending request`);
      console.log(options);

      const response: Response<Payload<T>> = await client?.request(options);
      const status = response.status;
      let msg = "";
      if (status < 200 || (status >= 300 && status != 401 && status != 500)) {
        // 处理http错误，抛到业务代码
        msg = showStatus(status);
        if (typeof response.data === "string") {
          response.data = { msg };
        } else {
          response.data.msg = msg;
        }
        return response;
      } else if (status == 200) {
        return response;
      } else if (status == 500) {
        msg = showStatus(status);
        response.data = { msg: msg };
        return response;
      }
    } catch (err: any) {
      console.error(err);
      err.message = "请求超时或服务器异常，请检查网络或联系管理员！";
      // 错误抛到业务代码
      return Promise.reject(err);
    }
  }
}

const requestClient = new RequestClient();

export default requestClient;

export interface Payload<T> {
  code: string;
  msg: string;
  message: string;
  data: T;
}
