<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import Loading from "$lib/components/DataLoading.svelte";
    import EveningCallTable from "$lib/components/EveningCallTable.svelte";
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
    <div class="min-h-screen">
        <div class="flex flex-col p-20">
            <div class="text-center mb-10">
                <div class="text-3xl text-primary font-bold">
                    <h1>点呼表</h1>
                </div>
            </div>
            <div class="flex-auto">
                <MorningCallTable calls={calls.morning_calls} usePhoto={true}/>
            </div>
            <div class="divider divider-primary flex-auto my-20">
                前{calls.morning_calls.length}件：後{calls.evening_calls.length}件
            </div>
            <div class="flex-auto">
                <EveningCallTable calls={calls.evening_calls} usePhoto={true}/>
            </div>
        </div>
    </div>
{/await}
