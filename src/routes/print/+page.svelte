<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Loading from "$lib/components/Loading.svelte"
    import Print from "$lib/components/Print.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").DailyReportPrint}
             */
            const print = await invoke('load_print');
            return print;
        } catch {
            goto('/error');
        }

        return {
            daily_report: null,
            morning_call: null,
            evening_call: null,
            locations: [],
            waitings: [],
            loadings: [],
            restings: []
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
{:then print} 
    <div class="min-h-screen">
        <div class="p-20">
            <div class="flex-auto">
                <Print/>
            </div>
        </div>
    </div>
{/await}