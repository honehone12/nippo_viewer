<script>
    'use strict';
    
    import {method, good, done, datetime, date} from "$lib/display";

    /**
     * @type {{calls: import("$lib/api").MorningCall[]}}
     */
    let {calls} = $props();
</script>

<table class="table">
    <caption class="text-2xl text-primary mb-10">前点呼</caption>
    <thead>
        <tr>
            <th>日時</th>
            <th>点呼者</th>
            <th>名前</th>
            <th>免許証期限</th>
            <th>車両番号</th>
            <th>方法</th>
            <th>検査機</th>
            <th>Alc検査</th>
            <th>Alc写真</th>
            <th>体調検査</th>
            <th>車両検査</th>
        </tr>
    </thead>
    <tbody>
        {#each calls as m (m.id)}
            <tr>
                <td>{datetime(m.created_at)}</td>
                <td>{m.caller}</td>
                <td>{m.name}</td>
                <td>{date(m.license_expiration)}</td>
                <td>{m.car_number}</td>
                <td>{method(m.method)}</td>
                <td>{done(m.using_alc_checker)}</td>
                <td>{good(m.alc_check)}</td>
                <td>{done(!!m.alc_photo)}</td>
                <td>{good(m.health_check)}</td>
                <td>{good(m.car_check)}</td>
            </tr>
            <tr>
                <td>備考</td>
                <td colspan="9">{m.note}</td>
            </tr>
        {/each}
    </tbody>
</table> 