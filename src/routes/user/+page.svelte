<script>
    'use strict';

    import { goto } from "$app/navigation";
    import Loading from "$lib/components/DataLoading.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let user = $state("");
    let submitting = $state(false);

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Users}
             */
            const users = await invoke('load_users');
            return users;
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
        
        return !!user;
    }

    async function onclick() {
        if (!valid()) {
            return;
        }

        submitting = true;

        try {
            await invoke('set_query_user', {user});

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
                <div class="text-2xl mb-5">
                    <h1 >取得するユーザーを選択してください</h1>
                </div>
                <div>
                    <UserSelector admin={false} users={users.users} bind:user/>
                </div>
                <div class="mt-10">
                    <button 
                        class="btn btn-primary" 
                        disabled={!ready}
                        {onclick}
                    >OK</button>    
                </div>
                {#if users.admin}
                    <div class="text-2xl text-secondary mb-10 mt-15">
                        <h1 >管理者メニュー</h1>
                    </div>
                    <button 
                        class="btn btn-secondary mr-15" 
                        disabled={ready}
                        onclick={() => goto('/invite')}
                    >招待ページへ</button>  
                    <button 
                        class="btn btn-secondary" 
                        disabled={ready}
                        onclick={() => goto('/promote')}
                    >昇格ページへ</button>  
                {/if}
            {/await}
        </div>
    </div>
</div>