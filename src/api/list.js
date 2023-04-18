import service from "@/utils/requestUtil";

export function getCardList() {
    return service({
        url: `https://temp.ceobecanteen.top/canteen/cardList`,
        method: 'GET',
    })
}

export function getAnnouncementInfo(){
    return service({
        url: `/canteen/operate/announcement/list`,
        method: 'GET',
    })
}

export function getResourceInfo(){
    return service({
        url: `/canteen/operate/resource/get`,
        method: 'GET',
    })
}

export function getVideoList(){
    return service({
        url: `canteen/operate/video/list`,
        method: 'GET',
    })
}
