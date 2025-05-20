<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Auth from "$lib/pages/Auth.svelte";

    async function load() {
        try {
            await invoke('obtain_tkn');

            goto('/user');
        } catch {
            goto('/error');
        }
    }
</script>

<Auth load={load()}/>
