<script>
    'use strict';

    import { datetime } from "$lib/display";
 
    /**
     * @type {{waitings: import("$lib/api").Waiting[]}}
     */
    let {waitings} = $props();
</script>

<table class="table table-s">
    <caption class="text-2xl text-primary mb-10">待機（荷主都合による）</caption>
    <thead>
        <tr>
            <th>開始日時</th>
            <th>終了日時</th>
            <th>場所名</th>
            <th>住所</th>
        </tr>
    </thead>
    <tbody>
        {#each waitings as w (w.id)}
            <tr>
                <td>{datetime(w.created_at)}</td>
                <td>{datetime(w.updated_at)}</td>
                <td>{w.label}</td>
                <td>{w.address}</td>
            </tr>
            <tr>
                <td>備考</td>
                <td colspan="3">{w.note}</td>
            </tr>
        {/each}
    </tbody>
</table>
