<script>
    'use strict';

    import {goto} from "$app/navigation";
    import {invoke} from "@tauri-apps/api/core";
    import Loading from "$lib/components/DataLoading.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";

    let user = $state("");
    let submitting = $state(false);
    let invited = $state("");

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Users}
             */
            const users = await invoke('load_users');

            if (users.admin) {
                return users.users.filter((u) => u.invitable).map((u) => u.user)
            }
            
            goto('/error');
        } catch {
            goto('/error');
        }

        return [];
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
            invited = await invoke('invite', {user});
            submitting = false;
            user = "";
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
                {#if submitting}
                    <Loading/>
                {:else}
                    {#if !invited}
                        <div class="mb-10">
                            <h1 class="text-2xl mb-5">招待するユーザーを選択してください</h1>
                            <p>ユーザーを招待するとにっぽーViewerにログインできるようになります</p>
                        </div>
                        <div>
                            <UserSelector 
                                admin={true} 
                                {users} 
                                bind:user
                            />
                        </div>
                        <div class="mt-10">
                            <button 
                                class="btn btn-secondary" 
                                disabled={!ready}
                                {onclick}
                            >OK</button>    
                        </div>
                    {:else}
                        <div class="text-xl mb-5">
                            <h1 >完了しました</h1>
                        </div>
                        <p>「{invited}」宛にメールを送信しました。</p>
                        <p>メールアドレスが間違っている場合は届きませんので、再度ラインで登録してください。</p>
                        <div class="mt-10">
                            <button 
                                class="btn btn-secondary" 
                                onclick={() => goto('/user')}
                            >OK</button>    
                        </div>
                    {/if}
                {/if} 
            {/await}
        </div>
    </div>
</div>