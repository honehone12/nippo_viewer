<script>
    'use strict';

    import LoadingDots from "$lib/components/LoadingDots.svelte"
    import DailyReport from "$lib/components/DailyReport.svelte";
    import Photo from "$lib/components/Photo.svelte";

    /**
     * @type {{load: Promise<{
     *  print: import("$lib/api").DailyReportPrint,
     *  photos: import("$lib/api").Photos
     * }>}}
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
{:then {print, photos}} 
    <div class="min-h-screen">
        <div class="p-20">
            <div class="mb-20">
                <DailyReport {print}/>
            </div>
            <div class="flex flex-row place-content-center">
                <Photo url={photos.morning_alc} name="前点呼アルコール検査"/>
                <Photo url={photos.evening_alc} name="後点呼アルコール検査"/>
                <Photo url={photos.morning_mtr} name="開始時メーター"/>
                <Photo url={photos.evening_mtr} name="終了時メーター"/>
            </div>
        </div>
    </div>
{/await}
