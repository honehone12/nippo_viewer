<script lang="ts">
    'use strict';
    
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Report from "$lib/pages/Report.svelte";
    import type { DailyReportMini } from "$lib/api";

    async function load() {
        try {
            const reports = await invoke<DailyReportMini[]>('load_reports');
            return reports;
        } catch {
            goto('/error');
        }

        return [];
    }

    async function onclickPrint(id: string) {
        try {
            await invoke('set_query_report', {report: id});
            goto('/print');
        } catch {
            goto('/error');
        }
    }

    async function onclickDownload(id: string) {
        try {
            await invoke('set_query_report', {report: id});
            goto('/download');
        } catch {
            goto('/error');
        }
    }
</script>

<Report load={load()} {onclickPrint} {onclickDownload}/>
