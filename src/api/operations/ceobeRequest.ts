import { getCardList } from "../resourceFetcher/cardList";
import { getConfigDatasourceList } from "../resourceFetcher/datasourceList";
import { getDatasourceComb } from "../resourceFetcher/datasourceCombine";
import { getCookieNewestInfo } from "../resourceFetcher/newestCookie";
import { getCookieList } from "../resourceFetcher/cookieList";
import { getAnnouncementInfo } from "../resourceFetcher/announcement";
import { getResourceInfo } from "../resourceFetcher/resource";
import { getHasRefererImageBase64, getLocalFileText } from "../function";

class CeobeRequest {
  getCardList = getCardList;
  getResourceList = getConfigDatasourceList;
  getDatasourceComb = getDatasourceComb;
  getDatasourceCombList = getCookieNewestInfo;
  getCookieList = getCookieList;
  getAnnouncementInfo = getAnnouncementInfo;
  getResourceInfo = getResourceInfo;

  getHasRefererImageBase64 = getHasRefererImageBase64;

  getLocalFileText = getLocalFileText;
}

const ceobeRequest = new CeobeRequest();

export default ceobeRequest;
