<script>
    'use strict';

    /**
     * @type {{url: string, name: string}}
     */
    let {url, name} = $props();
</script>

<div class="mx-5 text-center text-primary">
    <h2 class="text-xl mb-5">{name}</h2>
    {#if url}
        <figure>
            <img src="{url}" class="max-w-72" alt="{name}"/>
        </figure>
    {:else}
        <p>写真無し</p>
    {/if}
</div>