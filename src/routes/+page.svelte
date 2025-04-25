<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import AdminForm from "$lib/components/AdminForm.svelte";
    import Loading from "$lib/components/Loading.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    let orgId = $state('');
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
            const done = await listen('auth_done', () => {
                done();
                goto('/auth');
            });
            const failed = await listen('auth_failed', () => {
                failed();
                goto('/error');
            });
            
            await invoke('start_auth', {orgId});
        } catch (e) {
            console.error(e);
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
                <div class="text-2xl mb-10">
                    <h1 >管理者認証を行います（ブラウザが開きます）</h1>
                </div>
                <div class="mb-10">
                    <AdminForm bind:orgId disabled={submitting}/>
                </div>
                <div>
                    <button 
                        class="btn btn-primary" 
                        disabled={submitting || !orgId}
                        {onclick}
                    >OK</button>    
                </div>    
            {/await}
        </div>
    </div>
</div>
