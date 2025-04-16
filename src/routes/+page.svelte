<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import AdminForm from "$lib/components/AdminForm.svelte";
    import Loading from "$lib/components/Loading.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let orgId = $state('');
    let adminId = $state('');
    let adminPw = $state('');
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

    function valid() {
        if (submitting) {
            return false;
        }
        if (orgId.length === 0 || adminId.length === 0 || adminPw.length === 0) {
            return false;
        }

        return true;
    }

    async function onclick() {
        if (!valid()) {
            return;
        }
        submitting = true;

        try {
            await invoke('admin_auth', {
                orgId,
                adminId,
                adminPw
            });

            goto('/user');
        } catch {
            goto('/error');
        }
    }

    let ready = $derived(valid()); 
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                <Loading/>
            {:then} 
                <div class="text-2xl mb-5">
                    <h1 >管理者認証を行います</h1>
                </div>
                <AdminForm 
                    disabled={submitting}
                    bind:orgId 
                    bind:adminId
                    bind:adminPw
                />
                <div class="mt-5">
                    <button 
                        class="btn btn-primary" 
                        disabled={!ready}
                        {onclick}
                    >OK</button>    
                </div>    
            {/await}
        </div>
    </div>
</div>
