
import {getResourceList, getDatasourceComb, getCookieNewestInfo, getCookieList} from '@/api/list';

let old_cookie_id, old_update_cookie_id;
let datasource_comb_id;

export async function backgroundInit() {
    // 获取内存组合id配置
    datasource_comb_id = window.localStorage.getItem("datasource-comb");
    if(!datasource_comb_id) {
        // 获取数据源列表
        let resource_data = await getResourceList();
        let uuids = resource_data.data.data.map((item) => {
            return item.unique_id;
        });
        // 获取数据源组合id
        let datasource_comb_id_resp = await getDatasourceComb(uuids);
        datasource_comb_id = datasource_comb_id_resp.data.data.datasource_comb_id;
        window.localStorage.setItem("datasource-comb", datasource_comb_id);
    }
    window.datasourceConfig.datasourceCombUpdated(() => {
        datasource_comb_id = window.localStorage.getItem("datasource-comb");
        old_cookie_id = null;
        old_update_cookie_id = null;
    })
    await tryFetchCookie()
}

async function tryFetchCookie() {
    // 获取最新饼id信息
    let new_cookie_info  = await getCookieNewestInfo(datasource_comb_id);
    let {cookie_id, update_cookie_id} = new_cookie_info.data;

    if (cookie_id !== old_cookie_id && update_cookie_id !== old_update_cookie_id) {
        let cookies_data = await getCookieList(datasource_comb_id, cookie_id, update_cookie_id);
        let cookies_info = cookies_data.data.data;
        old_cookie_id = cookie_id;
        old_update_cookie_id = update_cookie_id;
        cookies_info.comb_id = datasource_comb_id;
        cookies_info.update_cookie_id = update_cookie_id;

        window.newestTimeline.sendTimeline(cookies_info);
    }

    setTimeout(async () => {
        await tryFetchCookie()
    }, 15*1000)
}