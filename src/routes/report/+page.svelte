<script>
    'use strict';

    import Loading from "$lib/components/Loading.svelte";

    async function load() {

    }
</script>

{#await load()}
    <div class="hero min-h-screen">
        <div class="hero-content text-center">
            <div class="p-20">
                <Loading/>
            </div>
        </div>
    </div>                
{:then reports} 
                
{/await}



