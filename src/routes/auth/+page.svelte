<script>
    'use strict';

    import { goto } from "$app/navigation";
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
                <p>認証中です</p>
            {:then} 
                <p>しばらく経ってもページが自動的に遷移しない場合は再起動してください</p>
            {/await}
        </div>
    </div>
</div>