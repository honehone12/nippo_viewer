<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";

    async function load() {
        try {
            const print = await invoke('load_print');
            return print;
        } catch {
            goto('/error');
        }
    }
</script>