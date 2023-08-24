class SearchWordEvent {
  getSearchWord<T>(callback: (event: string, payload: T) => void) {}
  sendSearchWord(data: string) {}
}

const searchWordEvent = new SearchWordEvent();

export default searchWordEvent;
