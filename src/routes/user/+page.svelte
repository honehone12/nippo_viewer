<script>
    'use strict';

    import { goto } from "$app/navigation";
    import Loading from "$lib/components/Loading.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let user = $state("");
    let submitting = $state(false);

    async function load() {
        try {
            /**
             * @type {Array<{
             *  id: string,
             *  name: string,
             * }>}
             */
            const users = await invoke('load_users');
            return users;
        } catch {
            goto('/error');
        }

        return [];
    }

    function valid() {
        if (submitting) {
            return false;
        }
        if (user.length == 0) {
            return false;
        }

        return true;
    }

    function onclick() {
        if (!valid()) {
            return;
        }

        submitting = true;
    }

    let ready = $derived(valid());
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                <Loading/>
            {:then users} 
                <div class="text-2xl mb-5">
                    <h1 >取得するユーザーを選択してください</h1>
                </div>
                <div>
                    <UserSelector {users} bind:user/>
                </div>
                <div class="mt-10">
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