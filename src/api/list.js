import service from "@/utils/request";

export const getCardList = () => {
    return service({
        url: `/canteen/cardList`,
        method: 'GET',
    })
}