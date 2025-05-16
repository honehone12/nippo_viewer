<script>
    'use strict';
    
    import { datetime, done } from "$lib/display";
    import EveningCallTable from "./EveningCallTable.svelte";
    import LoadingTable from "./LoadingTable.svelte";
    import LocationTable from "./LocationTable.svelte";
    import MorningCallTable from "./MorningCallTable.svelte";
    import RestingTable from "./RestingTable.svelte";
    import SiteTable from "./SiteTable.svelte";
    import WaitingTable from "./WaitingTable.svelte";

    /**
     * @type {{print: import("$lib/api").DailyReportPrint}}
     */
    let {print} = $props();

    function dutyDist() {
        if (!print.daily_report) {
            return 0;
        }

        const r = print.daily_report;
        return r.evening_meter - r.morning_meter - r.non_duty_distance;
    }
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
                    <th>開始メーター写真</th>
                    <th>終了メーター写真</th>
                    <th>終了メーター</th>
                    <th>開始メーター</th>
                    <th>空走行距離</th>
                    <th>走行距離</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{datetime(print.daily_report.created_at)}</td>
                    <td>{datetime(print.daily_report.updated_at)}</td>
                    <td>{print.daily_report.name}</td>
                    <td>{print.daily_report.car_number}</td>
                    <td>{done(!!print.daily_report.morning_meter_photo)}</td>
                    <td>{done(!!print.daily_report.evening_meter_photo)}</td>
                    <td>{print.daily_report.morning_meter}</td>
                    <td>{print.daily_report.evening_meter}</td>
                    <td>{print.daily_report.non_duty_distance}</td>
                    <td>{dutyDist()}</td>
                </tr>
                <tr>
                    <td>遅延・事故</td>
                    <td colspan="9">{print.daily_report.trouble}</td>
                </tr>
                <tr>
                    <td>備考</td>
                    <td colspan="9">{print.daily_report.note}</td>
                </tr>
            </tbody>
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
        {#if print.sites.length > 0}
            <div class="mb-10">
                <SiteTable sites={print.sites}/>
            </div>
        {:else}
            <div class="text-center mb-10">
                <p>現場の登録無し</p>
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
