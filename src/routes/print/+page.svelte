<script>
    'use strict';

    import { goto } from "$app/navigation";
    import Print from "$lib/pages/Print.svelte";
    import { invoke } from "@tauri-apps/api/core";
    
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

<Print load={load()}/>
