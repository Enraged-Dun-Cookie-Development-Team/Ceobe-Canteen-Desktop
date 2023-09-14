import {
  emit,
  EventCallback,
  listen,
  once,
  UnlistenFn,
} from "@tauri-apps/api/event";
import { Cookie, CookieList } from "../resourceFetcher/cookieList";

export interface TimelineData extends CookieList {
  comb_id: string;
  update_cookie_id: string | null;
}

class NewestTimeline {
  async getTimeline(
    callback: (event: string, payload: TimelineData) => void,
  ): Promise<UnlistenFn> {
    return await listen<Timeline>(
      "newest-timeline",
      (event: EventCallback<Timeline>) => {
        callback(event.event, event.payload);
      },
    );
  }

  async sendTimeline(data: TimelineData) {
    console.log("sending");
    console.log(data);
    await emit("newest-timeline", data);
  }

  async knowNeedTimeline(
    callback: (event: string) => void,
  ): Promise<UnlistenFn> {
    return await listen<void>("need-timeline", (event: EventCallback<void>) => {
      callback(event.payload);
    });
  }

  async needTimeline() {
    await emit<void>("need-timeline");
  }
}

const newestTimeline = new NewestTimeline();

export interface Timeline {
  timelineData: Cookie[] | null; // 饼列表
  refreshTimelineData: Cookie[] | null; // 刷新临时存的饼列表
  tempTimelineData: Cookie[] | null; // 用于搜索状态，临时存的饼列表
  nextPageId: string | null; // 下一页id
  refreshNextPageId: string | null; // 刷新临时存的下一页id
  tempNextPageId: string | null; // 用于搜索状态，临时存的下一页id
  combId: string | null; // 数据源组合id
  refreshCombId: string | null; // 刷新临时存的数据源组合id
  updateCookieId: string | null; // 更新饼id
  refreshUpdateCookieId: string | null; // 刷新临时存的更新饼id
  searchStatus: boolean | null; // 搜索状态，列表展示搜索内容就为true
  searchWord: string | null;
}

export default newestTimeline;
