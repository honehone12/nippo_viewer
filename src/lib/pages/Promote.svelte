<script>
    'use strict';

    import {goto} from "$app/navigation";
    import LoadingDots from "$lib/components/LoadingDots.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";

    /**
     * @type {{
     *  load: Promise<{
     *   admin: boolean,
     *   users: import("$lib/api").User[]
     *  }>,
     *  promote: (user: string) => Promise<void>
     * }}
     */
    let {load, promote} = $props();

    let user = $state("");
    let submitting = $state(false);
    let promoted = $state(false);

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

        await promote(user);

        promoted = true;
        user = "";
        submitting = false;
    }

    let ready = $derived(valid());
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load}
                <LoadingDots/>
            {:then {admin, users}}
                {#if admin}
                    {#if submitting}
                        <LoadingDots/>
                    {:else}
                        {#if !promoted}
                            <div class="mb-10">
                                <h1 class="text-2xl text-accent mb-5">昇格するユーザーを選択してください</h1>
                                <p>ユーザーを昇格させると組織内のすべてのユーザーデータを閲覧できます</p>
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
                                    class="btn btn-accent" 
                                    disabled={!ready}
                                    {onclick}
                                >OK</button>    
                            </div>
                        {:else}
                            <div class="text-xl mb-5">
                                <h1>完了しました</h1>
                            </div>
                            <div class="mt-10">
                                <button 
                                    class="btn btn-secondary" 
                                    onclick={() => goto('/user')}
                                >OK</button>    
                            </div>
                        {/if}
                    {/if}
                {/if} 
            {/await}
        </div>
    </div>
</div>
