<script>
    import EveningCallTable from "./EveningCallTable.svelte";
    import LocationTable from "./LocationTable.svelte";

    'use strict';
    
    import MorningCallTable from "./MorningCallTable.svelte";
    import WaitingTable from "./WaitingTable.svelte";

    /**
     * @type {{print: import("$lib/api").DailyReportPrint}}
     */
    let {print} = $props();
</script>

{#if print.daily_report}
    <div>
        {#if print.morning_call}
            <div class="mb-10">
                <MorningCallTable calls={[print.morning_call]}/>
            </div>    
        {/if}
        {#if print.evening_call}
            <div class="mb-10">
                <EveningCallTable calls={[print.evening_call]}/>
            </div>
        {/if}
        <div class="mb-10">
            <LocationTable locations={print.locations}/>
        </div>
        <div class="mb-10">
            <WaitingTable waitings={print.waitings}/>
        </div>
    </div>
{:else}
    <p>日報が存在しません</p>
{/if}