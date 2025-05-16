<script>
    'use strict';

    import {datetime} from "$lib/display";

    /**
     * @type {{restings: import("$lib/api").Resting[]}}
     */
    let {restings} = $props();
</script>

<table class="table">
    <caption class="text-2xl text-primary mb-10">休憩</caption>
    <colgroup>
        <col style="width: 15%;">
        <col style="width: 20%;">
        <col style="width: 25%;">
        <col style="width: 40%;">
    </colgroup>
    <thead>
        <tr>
            <th>登録日時</th>
            <th>場所名</th>
            <th>住所</th>
            <th>説明</th>
        </tr>
    </thead>
    <tbody>
        {#each restings as r (r.id)}
            <tr>
                <td>{datetime(r.created_at)}</td>
                <td>{r.label}</td>
                <td>{r.address}</td>
                <td>{r.short_note}</td>
            </tr>
        {/each}
    </tbody>
</table>