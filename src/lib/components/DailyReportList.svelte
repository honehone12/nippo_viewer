<script>
    'use strict'
    
    import {datetime} from "$lib/display"

    /**
     * @type {{
     *  reports: import("$lib/api").DailyReportMini[],
     *  onclickPrint: (id: string) => Promise<void>,
     *  onclickDownload: (id: string) => Promise<void>
     * }}
     */
    let {
        reports,
        onclickPrint,
        onclickDownload,
    } = $props(); 
</script>

<table class="table">
    <caption class="text-2xl text-primary mb-10">日報一覧</caption>
    <thead>
        <tr>
            <th>開始</th>
            <th>終了</th>
            <th>詳細</th>
            <th>写真</th>
        </tr>
    </thead>
    <tbody>
        {#each reports as r (r.id)}
            <tr>
                <td>{datetime(r.created_at)}</td>
                <td>{datetime(r.updated_at)}</td>
                <td>
                    <button 
                        class="btn btn-outline btn-primary" 
                        onclick="{() => onclickPrint(r.id)}"
                    >詳細を見る</button>
                </td>
                <td>
                    <button 
                        class="btn btn-outline btn-primary"
                        onclick="{() => onclickDownload(r.id)}"
                    >ダウンロードする</button>
                </td>
            </tr>
        {/each}
    </tbody>
</table>
