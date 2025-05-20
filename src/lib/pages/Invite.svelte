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
     *  invite: (user: string) => Promise<string>
     * }}
     */
    let {load, invite} = $props();

    let user = $state("");
    let submitting = $state(false);
    let invited = $state("");

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

        invited = await invite(user);

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
                        {#if !invited}
                            <div class="mb-10">
                                <h1 class="text-2xl text-accent mb-5">招待するユーザーを選択してください</h1>
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
                                    class="btn btn-accent" 
                                    disabled={!ready}
                                    {onclick}
                                >OK</button>    
                            </div>
                        {:else}
                            <div class="text-xl mb-5">
                                <h1>完了しました</h1>
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
                {/if}
            {/await}
        </div>
    </div>
</div>
