<script>
    'use strict';

    import {method, good, done, datetime} from "$lib/display";

    /**
     * @type {{calls: import("$lib/api").EveningCall[]}}
     */
    let {calls} = $props();
</script>

<table class="table table-s">
    <caption class="text-2xl text-primary mb-10">後点呼</caption>
    <thead>
        <tr>
            <th>日時</th>
            <th>点呼者</th>
            <th>名前</th>
            <th>車両番号</th>
            <th>方法</th>
            <th>検査機</th>
            <th>Alc検査</th>
            <th>写真</th>
        </tr>
    </thead>
    <tbody>
        {#each calls as e (e.id)}
            <tr>
                <td>{datetime(e.created_at)}</td>
                <td>{e.caller}</td>
                <td>{e.name}</td>
                <td>{e.car_number}</td>
                <td>{method(e.method)}</td>
                <td>{done(e.using_alc_checker)}</td>
                <td>{good(e.alc_check)}</td>
                <td>{done(!!e.alc_photo)}</td>
            </tr>
            <tr>
                <td>備考</td>
                <td colspan="7">{e.note}</td>
            </tr>
        {/each}
    </tbody>
</table>