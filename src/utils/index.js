export function getImg(url) {
    if (url) {
        return new URL(url, import.meta.url).href;
    } else {
        return null
    }
}