/**
     * @param {number} code 
     */
export function method(code) {
    switch (code) {
        case 1:
            return '対面';
        case 2:
            return '遠隔';
        default:
            return '不明';
    }
}

/**
     * @param {boolean} flag 
     */
export function done(flag) {
    return flag ? '有' : '無';
}

/**
 * @param {boolean} flag
 */
export function good(flag) {
    return flag ? '良' : '不'
}

/**
 * @param {string} enc
 */
export function datetime(enc) {
    try {
        const date = new Date(enc);
        const Y = date.getFullYear();
        const M = date.getMonth() + 1;
        const D = date.getDate();
        const H = date.getHours();
        const m = date.getMinutes();

        return `${Y}年${M}月${D}日${H}時${m}分`;   
    } catch {
        return '不明';
    }
}
