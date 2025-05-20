<script>
    'use strict';

    import DailyReportList from "$lib/components/DailyReportList.svelte";
    import LoadingDots from "$lib/components/LoadingDots.svelte";

    /**
     * @type {{
     *  load: Promise<import("$lib/api").DailyReportMini[]>,
     *  onclickPrint: (id: string) => Promise<void>,
     *  onclickDownload: (id: string) => Promise<void>
     * }}
     */
    let {load, onclickPrint, onclickDownload} = $props();
</script>

{#await load}
    <div class="hero min-h-screen">
        <div class="hero-content text-center">
            <div class="p-20">
                <LoadingDots/>
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
