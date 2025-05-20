<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Call from "$lib/pages/Call.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Calls}
             */
            const calls = await invoke('load_calls');
            return calls;  
        } catch {
            goto('/error');
        }

        return {
            morning_calls: [],
            evening_calls: []
        };
    }
</script>

<Call load={load()}/>
