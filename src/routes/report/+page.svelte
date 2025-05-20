<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Report from "$lib/pages/Report.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").DailyReportMini[]}
             */
            const reports = await invoke('load_reports');
            return reports;
        } catch {
            goto('/error');
        }

        return [];
    }

    /**
     * @param {string} id
     */
    async function onclickPrint(id) {
        try {
            await invoke('set_query_report', {report: id});
            goto('/print');
        } catch {
            goto('/error');
        }
    }

    /**
     * @param {string} id
     */
    async function onclickDownload(id) {
        try {
            await invoke('set_query_report', {report: id});
            goto('/download');
        } catch {
            goto('/error');
        }
    }
</script>

<Report load={load()} {onclickPrint} {onclickDownload}/>
