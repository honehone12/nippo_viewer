<script>
    'use strict';

    import {datetime} from "$lib/display";

    /**
     * @type {{sites: import("$lib/api").Site[]}}
     */
    let {sites} = $props();
</script>

<table class="table">
    <caption class="text-2xl text-primary mb-10">現場</caption>
    <thead>
        <tr>
            <th>登録日時</th>
            <th>名前</th>
            <th>説明</th>
        </tr>
    </thead>
    <tbody>
        {#each sites as s (s.id)}
            <tr>
                <td>{datetime(s.created_at)}</td>
                <td>{s.name}</td>
                <td>{s.short_note}</td>
            </tr>
        {/each}
    </tbody>
</table>
