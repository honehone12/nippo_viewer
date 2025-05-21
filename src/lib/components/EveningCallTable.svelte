<script>
    'use strict';

    import {method, good, exists, datetime, photo} from "$lib/display";

    /**
     * @type {{calls: import("$lib/api").EveningCall[], usePhoto: boolean}}
     */
    let {calls, usePhoto} = $props();
</script>

<table class="table">
    <caption class="text-2xl text-primary mb-10">後点呼</caption>
    <colgroup>
        <col style="width: 15%;">
    </colgroup>
    <thead>
        <tr>
            <th>日時</th>
            <th>点呼者</th>
            <th>名前</th>
            <th>車両番号</th>
            <th>方法</th>
            <th>検査機</th>
            <th>Alc検査</th>
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
                <td>{exists(e.using_alc_checker)}</td>
                <td>{good(e.alc_check)}{#if usePhoto}{photo(e.alc_photo)}{/if}</td>
            </tr>
            <tr>
                <td>備考</td>
                <td colspan="6">{e.note}</td>
            </tr>
        {/each}
    </tbody>
</table>