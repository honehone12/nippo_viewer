<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Loading from "$lib/components/DataLoading.svelte"
    import Print from "$lib/components/Print.svelte";
    import Photo from "$lib/components/Photo.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").DailyReportPrint}
             */
            const print = await invoke('load_print');
            /**
             * @type {import("$lib/api").Photos}
             */
            const photos = await invoke('load_download');

            return {print, photos};
        } catch {
            goto('/error');
        }

        return {
            print: {
                daily_report: null,
                morning_call: null,
                evening_call: null,
                sites: [],
                locations: [],
                waitings: [],
                loadings: [],
                restings: []
            },
            photos: {
                morning_alc: '',
                evening_alc: '',
                morning_mtr: '',
                evening_mtr: ''
            }
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
{:then {print, photos}} 
    <div class="min-h-screen">
        <div class="p-20">
            <div class="mb-20">
                <div class="">
                    <Print {print}/>
                </div>
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
