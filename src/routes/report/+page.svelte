<script lang="ts">
    'use strict';
    
    import { goto } from "$app/navigation";
    import {
        type MorningCall, 
        type EveningCall
    } from "$lib/api";
    import Loading from "$lib/components/Loading.svelte";
    import { invoke } from "@tauri-apps/api/core";

    interface Calls {
        morning_calls: Array<MorningCall>,
        evening_calls: Array<EveningCall>
    }

    async function load() {
        try {
            const calls: Calls = await invoke('load_calls');
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
                
            {/await}
        </div>
    </div>
</div>
