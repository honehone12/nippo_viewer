<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Download from "$lib/pages/Download.svelte";
    
    async function load() {
        try {
            /**
             * @type {import("$lib/api").Photos}
             */
            const photos = await invoke('load_download');
            return photos;
        } catch {
            goto('/error');
        }

        return {
            morning_alc: '',
            evening_alc: '',
            morning_mtr: '',
            evening_mtr: ''
        };
    }
</script>

<Download load={load()}/>
