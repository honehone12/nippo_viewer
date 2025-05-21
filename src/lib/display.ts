export function method(code: number) {
    switch (code) {
        case 1:
            return '対面';
        case 2:
            return '遠隔';
        default:
            return '不明';
    }
}

export function exists(flag: boolean) {
    return flag ? '有' : '無';
}

export function good(flag: boolean) {
    return flag ? '良' : '不'
}

export function checked(flag: boolean) {
    return flag ? '済' : '不'
}

export function datetime(enc: string) {
    try {
        const date = new Date(enc);
        const Y = date.getFullYear();
        const M = date.getMonth() + 1;
        const D = date.getDate();
        const H = date.getHours();
        const m = date.getMinutes();

        if (Y === 1) {
            return '';
        }

        return `${Y}年${M}月${D}日${H}時${m}分`;   
    } catch {
        return '';
    }
}

export function date(enc: string) {
    try {
        const date = new Date(enc);
        const Y = date.getFullYear();
        const M = date.getMonth() + 1;
        const D = date.getDate();

        if (Y === 1) {
            return '';
        }

        return `${Y}年${M}月${D}日`;   
    } catch {
        return '';
    }
}

export function photo(photoName: string) {
    return `（写真${photoName ? '有' : '無'}）`
}
