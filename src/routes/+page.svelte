<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import Loading from "$lib/components/Loading.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let submitting = $state(false);

    async function load() {
        try {
            const exists = await invoke('exists_auth');

            if (exists) {
                goto('/user');
            }
        } catch {
            goto('/error');
        }
    }

    async function onclick() {
        if (submitting) {
            return;
        }

        submitting = true;

        try {
            await invoke('start_auth');

            goto('/auth');
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
            {:then}
                <div class="text-2xl mb-5">
                    <h1 >管理者認証を行います（ブラウザが開きます）</h1>
                </div>
                <div class="mt-5">
                    <button 
                        class="btn btn-primary" 
                        disabled={submitting}
                        {onclick}
                    >OK</button>    
                </div>    
            {/await}
        </div>
    </div>
</div>
