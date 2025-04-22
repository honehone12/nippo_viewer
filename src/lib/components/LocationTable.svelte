<script>
    'uset strict';
    
    import { datetime } from "$lib/display";

    /**
     * @type {{locations: import("$lib/api").Location[]}}
     */
    let {locations} = $props()
</script>

<table class="table table-s">
    <caption class="text-2xl text-primary mb-10">位置情報・ルート履歴</caption>
    <thead>
        <tr>
            <th>登録日時</th>
            <th>場所名</th>
            <th>住所</th>
            <th>緯度</th>
            <th>経度</th>
        </tr>
    </thead>
    <tbody>
        {#each locations as l (l.id)}
            <tr>
                <td>{datetime(l.created_at)}</td>
                <td>{l.label}</td>
                <td>{l.address}</td>
                <td>{l.latitude.toFixed(4)}</td>
                <td>{l.longitude.toFixed(4)}</td>
            </tr>
            <tr>
                <td>説明</td>
                <td colspan="4">{l.short_note}</td>
            </tr>
        {/each}
    </tbody>
</table>