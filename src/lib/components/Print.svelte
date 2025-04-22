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
    <div class="mb-10">
        <table class="table table-m">
            <caption class="text-3xl font-bold text-primary mb-10">日報</caption>
        </table>
    </div>
    <div>
        {#if print.morning_call}
            <div class="mb-10">
                <MorningCallTable calls={[print.morning_call]}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>前点呼無し</p>
            </div>    
        {/if}
        {#if print.evening_call}
            <div class="mb-10">
                <EveningCallTable calls={[print.evening_call]}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>後点呼無し</p>
            </div>   
        {/if}
        {#if print.locations.length > 0}
            <div class="mb-10">
                <LocationTable locations={print.locations}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>位置情報の登録無し</p>
            </div>    
        {/if}
        {#if print.waitings.length > 0}
            <div class="mb-10">
                <WaitingTable waitings={print.waitings}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>待機の登録無し</p>
            </div>    
        {/if}
        {#if print.loadings.length > 0}
            <div class="mb-10">
                <LoadingTable loadings={print.loadings}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>荷役・附帯業務の登録無し</p>
            </div>    
        {/if}
        {#if print.restings.length > 0}
            <div class="mb-10">
                <RestingTable restings={print.restings}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>休憩の登録無し</p>
            </div>
        {/if}
    </div>
{:else}
    <div class="text-center">
        <p>日報無し</p>
    </div>
{/if}