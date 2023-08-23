import service from '@/utils/requestUtil';

/**
 * 获取公告
 * @returns {*}
 */
export function getAnnouncementInfo() {
  return service({
    urlChoice: 'SERVER_URL',
    url: `/canteen/operate/announcement/list`,
    method: 'GET',
  });
}

export function getResourceInfo() {
  return service({
    urlChoice: 'SERVER_URL',
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
    urlChoice: 'SERVER_URL',
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
    urlChoice: 'SERVER_URL',
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
    urlChoice: 'SERVER_URL',
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
    urlChoice: 'CDN_URL',
    url: `/datasource-comb/${comb_id}`,
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
    urlChoice: 'CDN_SERVER_URL',
    url: `/cdn/cookie/mainList/cookieList`,
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
    urlChoice: 'SERVER_URL',
    url: `/canteen/config/datasource/list`,
    method: 'GET',
  });
}

/**
 * 获取饼搜索列表
 * @returns {*}
 */
export function getCookieSearchList(cookie_id, datasource_comb_id, search_word) {
  let params = {
    datasource_comb_id: datasource_comb_id,
    search_word: search_word,
  };

  if (cookie_id) {
    params['cookie_id'] = cookie_id;
  }
  return service({
    urlChoice: 'SERVER_URL',
    url: `/canteen/cookie/search/list`,
    method: 'GET',
    params: params,
  });
}

/**
 * 获取饼搜索列表
 * @returns {*}
 */
export function getVersion(version) {
  let params = {};

  if (version) {
    params['version'] = version;
  }
  return service({
    urlChoice: 'SERVER_URL',
    url: `/canteen/operate/version/desktop`,
    method: 'GET',
    params: params,
  });
}
