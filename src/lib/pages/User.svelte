<script>
    'use strict';

    import { goto } from "$app/navigation";
    import LoadingDots from "$lib/components/LoadingDots.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";

    /**
     * @type {{
     *  load: Promise<{
     *   admin: boolean, 
     *   users: import("$lib/api").User[]
     *  }>,
     *  selectUser: (user: string) => Promise<void>
     * }}
     */
    let {load, selectUser} = $props();

    let user = $state("");
    let submitting = $state(false);

    function valid() {
        return !submitting && !!user;
    }

    async function onclick() {
        if (!valid()) {
            return;
        }

        submitting = true;

        await selectUser(user);
    }

    let ready = $derived(valid());
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load}
                <LoadingDots/>
            {:then {admin, users}} 
                <div class="text-2xl text-primary mb-5">
                    <h1 >取得するユーザーを選択してください</h1>
                </div>
                <div>
                    <UserSelector admin={false} {users} bind:user/>
                </div>
                <div class="mt-10">
                    <button 
                        class="btn btn-primary" 
                        disabled={!ready}
                        {onclick}
                    >OK</button>    
                </div>
                {#if admin}
                    <div class="text-2xl text-accent mb-10 mt-20">
                        <h1 >管理者メニュー</h1>
                    </div>
                    <button 
                        class="btn btn-accent mr-15" 
                        disabled={ready}
                        onclick={() => goto('/invite')}
                    >招待ページへ</button>  
                    <button 
                        class="btn btn-accent" 
                        disabled={ready}
                        onclick={() => goto('/promote')}
                    >昇格ページへ</button>  
                {/if}
            {/await}
        </div>
    </div>
</div>
