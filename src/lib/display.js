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
        const d = new Date(enc);
        return `${d.getFullYear()}/${d.getMonth()}/${d.getDay()} ${d.getHours()}:${d.getMinutes()}`;   
    } catch {
        return  '不明';
    }
}