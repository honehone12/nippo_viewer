<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import EveningCallTable from "$lib/components/EveningCallTable.svelte";
    import Loading from "$lib/components/Loading.svelte";
    import MorningCallTable from "$lib/components/MorningCallTable.svelte";
    import { invoke } from "@tauri-apps/api/core";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Calls}
             */
            const calls = await invoke('load_calls');
            return calls;  
        } catch {
            goto('/error');
        }

        return {
            morning_calls: [],
            evening_calls: []
        };
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
{:then calls}
    <div class="w-full p-20">
        <div class="flex flex-col">
            <MorningCallTable calls={calls.morning_calls}/>
            <div class="divider divider-primary my-10"></div>
            <EveningCallTable calls={calls.evening_calls}/>
        </div>
    </div>
{/await}
