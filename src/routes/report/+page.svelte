<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import Loading from "$lib/components/Loading.svelte";
    import { invoke } from "@tauri-apps/api/core";

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

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                <Loading/>
            {:then calls} 
                {#each calls.morning_calls as mcall (mcall.id)}
                    
                {/each}

                {#each calls.evening_calls as ecall (ecall.id)}
                    
                {/each}
            {/await}
        </div>
    </div>
</div>
