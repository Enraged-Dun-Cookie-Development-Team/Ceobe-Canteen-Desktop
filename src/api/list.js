import service from '@/utils/requestUtil';

export function getCardList() {
  return service({
    url: `https://temp.ceobecanteen.top/canteen/cardList`,
    method: 'GET'
  });
}

export function getAnnouncementInfo() {
  return service({
    url: `/canteen/operate/announcement/list`,
    method: 'GET'
  });
}

export function getResourceInfo() {
  return service({
    url: `/canteen/operate/resource/get`,
    method: 'GET'
  });
}

export function getVideoList() {
  return service({
    url: `/canteen/operate/video/list`,
    method: 'GET'
  });
}

export function getResourceList() {
  // 返回带uuid的资源列表
  return service({
    url: `/canteen/config/datasource/list`,
    method: 'GET'
  });
}

export function getDatasourceComb(uuids) {
  // uuid: list[uuid]
  //返回带有一个datasource_comb_id的json
  return service({
    url: `/canteen/user/getDatasourceComb`,
    method: 'POST',
    data: {
      datasource_push: uuids
    }
  });
}

export function getCookieNewestInfo(comb_id) {
  // comb_id: str
  // 返回带有cookie_id和update_cookie_id的json
  return service({
    url: `http://cdn-dev.ceobecanteen.top/datasource-comb/${comb_id}`,
    method: 'GET'
  });
}

export function getCookieList(comb_id, cookie_id, update_cookie_id) {
  // comb_id: str
  // cookie_id: str
  // update_cookie_id: str
  // 返回带有cookies的json
  return service({
    url: `http://cdn-muelsyse-dev.ceobecanteen.top/api/v1/cdn/cookie/mainList/cookieList`,
    params: {
      datasource_comb_id: comb_id,
      cookie_id: cookie_id,
      update_cookie_id: update_cookie_id
    },
    method: 'GET'
  });
}
