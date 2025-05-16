<script>
    'use strict';

    import { goto } from "$app/navigation";
    import Loading from "$lib/components/DataLoading.svelte";
    import { invoke } from "@tauri-apps/api/core";

    async function load() {
        try {
            await invoke('obtain_tkn');

            goto('/user');
        } catch {
            goto('/error');
        }
    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                <Loading/>
            {/await}
        </div>
    </div>
</div>