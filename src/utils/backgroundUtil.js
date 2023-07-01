
import { ipcRenderer } from 'electron';
import {getResourceList, getDatasourceComb, getDatasourceCombList, getCookieList} from '@/api/list';

let old_cookie_id, old_update_cookie_id;
let datasource_comb_id;

export async function backgroundInit() {
    let resource_data = await getResourceList();
    console.log(resource_data)
    let uuids = resource_data.data.data.map((item) => {
            return item.unique_id;
        });
    let datasource_comb_id_data = await getDatasourceComb(uuids);
    datasource_comb_id = datasource_comb_id_data.data.data.datasource_comb_id;
    await tryFetchCookie()
}

async function tryFetchCookie() {
    let new_cookie_info  = await getDatasourceCombList(datasource_comb_id);
    let {cookie_id, update_cookie_id} = new_cookie_info.data;

    if (cookie_id !== old_cookie_id && update_cookie_id !== old_update_cookie_id) {
        let cookies_data = await getCookieList(datasource_comb_id, cookie_id, update_cookie_id);
        let cookies_info = cookies_data.data.data;

        // ipcRenderer.send('newest-timeline', data);
        console.log('newest-timeline', cookies_info);
    }

    setTimeout(async () => {
        await tryFetchCookie()
    }, 15*1000)
}