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

    /**
     * @type {import("$lib/api").User[]}
     */
    let users = $state([])
    let user = $state("");
    let submitting = $state(false);
    let promoted = $state(false);

    async function init() {
        const {admin, users: u} = await load;
        users = u;
        return admin;
    }

    function valid() {
        if (submitting) {
            return false;
        }
        
        return !!user;
    }

    function findName() {
        const u = users.find((u) => u.id === user);
        return u ? u.name : '';
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

        await promote(user);

        promoted = true;
        user = "";
        submitting = false;
    }

    let ready = $derived(valid());
    let name = $derived(findName());
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await init()}
                <LoadingDots/>
            {:then admin}
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
                                    onclick={beforeclick}
                                    disabled={!ready}
                                >OK</button>
                                
                                <dialog id="promote_modal_dialog" class="modal">
                                    <div class="modal-box">
                                        <h3 class="text-lg text-accent font-bold">注意</h3>
                                        <div class="py-4">
                                            <p>昇格後の変更は出来ません</p>
                                            <p>十分に注意してください</p>
                                            <p>昇格対象者：　{name}</p>
                                        </div>
                                        <div class="modal-action place-content-center">
                                            <form method="dialog">
                                                <button 
                                                    class="btn btn-accent mr-5"
                                                    disabled={!ready}
                                                    {onclick} 
                                                    type="button"
                                                >続行</button>
                                                <button class="btn" type="button">戻る</button>
                                            </form>
                                        </div>
                                    </div>
                                </dialog>
                            </div>
                        {:else}
                            <div class="text-xl mb-5">
                                <h1>完了しました</h1>
                                <p>続けて昇格を行う場合は一度再起動してください</p>
                            </div>
                            <div class="mt-10">
                                <button 
                                    class="btn btn-accent" 
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
