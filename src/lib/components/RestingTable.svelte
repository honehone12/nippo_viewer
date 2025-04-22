<script>
    'use strict';

    import {datetime} from "$lib/display";

    /**
     * @type {{restings: import("$lib/api").Resting[]}}
     */
    let {restings} = $props();
</script>

<table class="table table-sm">
    <caption class="text-2xl text-primary mb-10">休憩</caption>
    <thead>
        <tr>
            <th>開始日時</th>
            <th>終了日時</th>
            <th>場所名</th>
            <th>住所</th>
        </tr>
    </thead>
    <tbody>
        {#each restings as r (r.id)}
            <tr>
                <td>{datetime(r.created_at)}</td>
                <td>{datetime(r.updated_at)}</td>
                <td>{r.label}</td>
                <td>{r.address}</td>
            </tr>
            <tr>
                <td>説明</td>
                <td colspan="3">{r.short_note}</td>
            </tr>
        {/each}
    </tbody>
</table>