import service from "@/utils/request";

export function getCardList() {
    return service({
        url: `/canteen/cardList`,
        method: 'GET',
    })
}