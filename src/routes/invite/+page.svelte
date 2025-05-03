<script>
    'use strict';

    import {goto} from "$app/navigation";
    import {invoke} from "@tauri-apps/api/core";
    import Loading from "$lib/components/Loading.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";

    let user = $state("");
    let submitting = $state(false);

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Users}
             */
            const users = await invoke('load_users');

            if (users.admin) {
                return users;
            }
            
            goto('/error');
        } catch {
            goto('/error');
        }

        return {
            admin: false,
            invitables: [],
            users: []
        };
    }

    function valid() {
        if (submitting) {
            return false;
        }
        
        return user.length !== 0;
    }

    async function onclick() {
        if (!valid()) {
            return;
        }

        submitting = true;

        try {
            await invoke('invite', {user});

            goto('/query');
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
            {:then users}
                {#if users.admin}
                    <div class="text-2xl mb-5">
                        <h1 >招待するユーザーを選択してください</h1>
                    </div>
                    <div>
                        <UserSelector users={users.users} bind:user/>
                    </div>
                    <div class="mt-10">
                        <button 
                            class="btn btn-primary" 
                            disabled={!ready}
                            {onclick}
                        >OK</button>    
                    </div>
                {/if} 
            {/await}
        </div>
    </div>
</div>