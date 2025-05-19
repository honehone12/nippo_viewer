<script>
    'use strict';

    /**
     * @type {{url: string, name: string}}
     */
    let {url, name} = $props();
</script>

<div class="mx-5 text-center">
    <div class="text-primary">
        <h2 class="text-xl mb-5">{name}</h2>
    </div>
    {#if url}
        <figure>
            <img src="{url}" class="max-w-70" alt="{name}"/>
        </figure>
    {:else}
        <p>写真無し</p>
    {/if}
</div>