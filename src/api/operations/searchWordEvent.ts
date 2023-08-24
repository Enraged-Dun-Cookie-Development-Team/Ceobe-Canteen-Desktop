import {
  emit,
  EventCallback,
  listen,
  UnlistenFn,
  Event,
} from "@tauri-apps/api/event";

class SearchWordEvent {
  async getSearchWord(
    callback: (event: string, payload: string) => void,
  ): Promise<UnlistenFn> {
    return await listen<string>("search-word", (event: Event<string>) => {
      callback(event.event, event.payload);
    });
  }
  async sendSearchWord(data: string) {
    await emit("search-word", data);
  }
}

const searchWordEvent = new SearchWordEvent();

export default searchWordEvent;
