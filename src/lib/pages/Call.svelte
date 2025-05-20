<script>
    'use strict';

    import LoadingDots from "$lib/components/LoadingDots.svelte";
    import EveningCallTable from "$lib/components/EveningCallTable.svelte";
    import MorningCallTable from "$lib/components/MorningCallTable.svelte";

    /**
     * @type {{load: Promise<import("$lib/api").Calls>}}
     */
    let {load} = $props();
</script>

{#await load}
    <div class="hero min-h-screen">
        <div class="hero-content text-center">
            <div class="p-20">
                <LoadingDots/>
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
            <div class="divider divider-primary text-primary flex-auto my-20">
                前{calls.morning_calls.length}件：後{calls.evening_calls.length}件
            </div>
            <div class="flex-auto">
                <EveningCallTable calls={calls.evening_calls} usePhoto={true}/>
            </div>
        </div>
    </div>
{/await}
