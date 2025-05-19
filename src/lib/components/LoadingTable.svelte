<script>
    'use strict';

    import { datetime, check } from "$lib/display";

    /**
     * @type {{loadings: import("$lib/api").Loading[]}}
     */
    let {loadings} = $props();
</script>

<table class="table">
    <caption class="text-2xl text-primary mb-10">荷役・附帯業務</caption>
    <colgroup>
        <col style="width: 15%;">
        <col style="width: 15%;">
    </colgroup>
    <thead>
        <tr>
            <th>開始日時</th>
            <th>終了日時</th>
            <th>場所名</th>
            <th>住所</th>
            <th>荷主確認</th>
        </tr>
    </thead>
    <tbody>
        {#each loadings as l (l.id)}
            <tr>
                <td>{datetime(l.created_at)}</td>
                <td>{datetime(l.updated_at)}</td>
                <td>{l.label}</td>
                <td>{l.address}</td>
                <td>{check(l.shipping_check)}</td>
            </tr>
            <tr>
                <td>備考</td>
                <td colspan="4">{l.note}</td>
            </tr>
        {/each}
    </tbody>
</table>