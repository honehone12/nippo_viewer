<script>
    'use strict';
    
    import { datetime, done } from "$lib/display";
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
        <table class="table">
            <caption class="text-3xl font-bold text-primary mb-10">日報</caption>
            <thead>
                <tr>
                    <th>開始日時</th>
                    <th>終了日時</th>
                    <th>名前</th>
                    <th>車両番号</th>
                    <th>メーター写真</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{datetime(print.daily_report.created_at)}</td>
                    <td>{datetime(print.daily_report.updated_at)}</td>
                    <td>{print.daily_report.name}</td>
                    <td>{print.daily_report.car_number}</td>
                    <td>{done(!!print.daily_report.meter_photo.length)}</td>
                </tr>
                <tr>
                    <td>遅延・事故</td>
                    <td colspan="4">{print.daily_report.trouble}</td>
                </tr>
                <tr>
                    <td>備考</td>
                    <td colspan="4">{print.daily_report.note}</td>
                </tr>
            </tbody>
        </table>
    </div>
    <div>
        {#if print.morning_call}
            <div tabindex="-1" class="collapse collapse-arrow bg-base-200 mb-10 print-exclude">
                <div class="collapse-title">前点呼</div>
                <div class="collapse-content">
                    <MorningCallTable calls={[print.morning_call]}/>
                    <div class="text-center mt-5">
                        <p>（印刷はされません）</p>
                    </div>
                </div>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>前点呼無し</p>
            </div>    
        {/if}
        {#if print.evening_call}
            <div tabindex="-1" class="collapse collapse-arrow bg-base-200 mb-10 print-exclude">
                <div class="collapse-title">後点呼</div>
                <div class="collapse-content">
                    <EveningCallTable calls={[print.evening_call]}/>
                    <div class="text-center mt-5">
                        <p>（印刷はされません）</p>
                    </div>
                </div>
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
            <div>
                <RestingTable restings={print.restings}/>
            </div>
        {:else}
            <div class="text-center">
                <p>休憩の登録無し</p>
            </div>
        {/if}
    </div>
{:else}
    <div class="text-center">
        <p>日報無し</p>
    </div>
{/if}

<style>
    @media print {
        .print-exclude {
            display: none;
        }
    }
</style>
