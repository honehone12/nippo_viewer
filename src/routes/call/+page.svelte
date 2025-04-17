<script lang="ts">
    'use strict';

    async function load() {

    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                
            {:then calls} 
                
            {/await}
        </div>
    </div>
</div>
