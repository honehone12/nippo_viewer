<script lang="ts">
    'use strict';
    
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Query from "$lib/pages/Query.svelte";
    
    async function setQuery(q: string, y: string, m: string) {
        try {
            await invoke('set_query_ym', {y, m});

            switch (q) {
                case 'reports':
                    goto('/report');
                    break;
                case 'calls':
                    goto('/call');
                    break;
                default:
                    goto('/error');
                    break;
            }
        } catch {
            goto('/error');
        }
    }
</script>

<Query {setQuery}/>
