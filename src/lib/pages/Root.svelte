<script>
    'use strict';

    import LoadingDots from "$lib/components/LoadingDots.svelte";

    /**
     * @type {{
     *  load: Promise<boolean>
     *  startAuth: () => Promise<void>
     * }}
     */
    let {load, startAuth} = $props()

    let submitting = $state(false);

    async function onclick() {
        if (submitting) {
            return;
        }

        submitting = true;

        await startAuth();
    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load}
                <LoadingDots/>
            {:then exists}
                {#if !exists}
                    <div class="text-2xl text-primary mb-10">
                        <h1 >閲覧者認証を行います（ブラウザが開きます）</h1>
                    </div>
                    <div>
                        <button 
                            class="btn btn-primary" 
                            disabled={submitting}
                            {onclick}
                        >OK</button>    
                    </div>    
                {/if}
            {/await}
        </div>
    </div>
</div>
