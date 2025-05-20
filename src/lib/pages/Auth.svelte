<script>
    'use strict';

    import LoadingDots from "$lib/components/LoadingDots.svelte";

    /**
     * @type {{load: Promise<void>}}
     */
    let {load} = $props();
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load}
                <LoadingDots/>
            {/await}
        </div>
    </div>
</div>