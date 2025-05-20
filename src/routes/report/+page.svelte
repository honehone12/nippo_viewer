<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import DailyReportList from "$lib/components/DailyReportList.svelte";
    import Loading from "$lib/components/DataLoading.svelte";
    import { invoke } from "@tauri-apps/api/core";

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

{#await load()}
    <div class="hero min-h-screen">
        <div class="hero-content text-center">
            <div class="p-20">
                <Loading/>
            </div>
        </div>
    </div>                
{:then reports} 
    <div class="min-h-screen">
        <div class="flex p-20">
            <div class="flex-auto">
                <DailyReportList
                    {reports}
                    {onclickPrint}
                    {onclickDownload}
                />
            </div>
        </div>
    </div>
{/await}
