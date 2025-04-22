<script>
    'use strict';

    import EveningCallTable from "./EveningCallTable.svelte";
    import LoadingTable from "./LoadingTable.svelte";
    import LocationTable from "./LocationTable.svelte";
    import MorningCallTable from "./MorningCallTable.svelte";
    import RestingTable from "./RestingTable.svelte";
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
        {#if print.locations.length > 0}
            <div class="mb-10">
                <LocationTable locations={print.locations}/>
            </div>
        {/if}
        {#if print.waitings.length > 0}
            <div class="mb-10">
                <WaitingTable waitings={print.waitings}/>
            </div>    
        {/if}
        {#if print.loadings.length > 0}
            <div class="mb-10">
                <LoadingTable loadings={print.loadings}/>
            </div>    
        {/if}
        {#if print.restings.length > 0}
            <div class="mb-10">
                <RestingTable restings={print.restings}/>
            </div>    
        {/if}
    </div>
{:else}
    <p>日報が存在しません</p>
{/if}