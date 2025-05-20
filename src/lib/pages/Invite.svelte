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

    function beforeclick() {
        const elem = document.getElementById('promote_modal_dialog');
        if (elem instanceof HTMLDialogElement) {
            elem.showModal();
        } else {
            goto('/error');
        }
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
                                    onclick={beforeclick}
                                    disabled={!ready}
                                >OK</button>
                                
                                <dialog id="promote_modal_dialog" class="modal">
                                    <div class="modal-box">
                                        <h3 class="text-lg text-accent font-bold">注意</h3>
                                        <div class="py-4">
                                            <p>招待後の取消は出来ません</p>
                                            <p>十分に注意してください</p>
                                            <p>招待者：　{users.find((u) => u.id === user)?.name}</p>
                                        </div>
                                        <div class="modal-action place-content-center">
                                            <form method="dialog">
                                                <button 
                                                    class="btn btn-accent mr-5"
                                                    disabled={!ready}
                                                    {onclick} 
                                                >続行</button>
                                                <button class="btn">戻る</button>
                                            </form>
                                        </div>
                                    </div>
                                </dialog>
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
