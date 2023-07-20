import service from '@/utils/requestUtil';

/**
 * 获取列表
 * @returns {*}
 */
export function getCardList() {
  return service({
    url: `https://temp.ceobecanteen.top/canteen/cardList`,
    method: 'GET',
  });
}

/**
 * 获取公告
 * @returns {*}
 */
export function getAnnouncementInfo() {
  return service({
    url: `/canteen/operate/announcement/list`,
    method: 'GET',
  });
}

export function getResourceInfo() {
  return service({
    url: `/canteen/operate/resource/get`,
    method: 'GET',
  });
}

/**
 * 视频推荐列表
 * @returns {*}
 */
export function getVideoList() {
  return service({
    url: `/canteen/operate/video/list`,
    method: 'GET',
  });
}

/**
 * 返回带uuid的资源列表
 * @returns {*}
 */
export function getResourceList() {
  return service({
    url: `/canteen/config/datasource/list`,
    method: 'GET',
  });
}

/**
 * 返回带有一个datasource_comb_id的json
 * @param uuids list[uuid]
 * @returns {*}
 */
export function getDatasourceComb(uuids) {
  return service({
    url: `/canteen/user/getDatasourceComb`,
    method: 'POST',
    data: {
      datasource_push: uuids,
    },
  });
}

/**
 * 返回带有cookie_id和update_cookie_id的json
 * @param comb_id string
 * @returns {*}
 */
export function getCookieNewestInfo(comb_id) {
  return service({
    url: `http://cdn-dev.ceobecanteen.top/datasource-comb/${comb_id}`,
    method: 'GET',
  });
}

/**
 * 返回带有cookies的json
 * @param comb_id string
 * @param cookie_id string
 * @param update_cookie_id string
 * @returns {*}
 */
export function getCookieList(comb_id, cookie_id, update_cookie_id) {
  return service({
    url: `http://cdn-muelsyse-dev.ceobecanteen.top/api/v1/cdn/cookie/mainList/cookieList`,
    params: {
      datasource_comb_id: comb_id,
      cookie_id: cookie_id,
      update_cookie_id: update_cookie_id,
    },
    method: 'GET',
  });
}

/**
 * 获取所有数据源信息
 * @returns {*}
 */
export function getAllDatasources() {
  return service({
    url: `/canteen/config/datasource/list`,
    method: 'GET',
  });
}
