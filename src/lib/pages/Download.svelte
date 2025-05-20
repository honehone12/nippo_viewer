<script>
    'use strict';

    import LoadingDots from '$lib/components/LoadingDots.svelte';
    import PhotoCard from "$lib/components/PhotoCard.svelte";

    /**
     * @type {{load: Promise<import("$lib/api").Photos>}}
     */
    let {load} = $props();
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load}
                <LoadingDots/>
            {:then photos}
                {#if photos.morning_alc}
                    <div class="mb-5">
                        <PhotoCard url={photos.morning_alc} name="前点呼アルコール検査"/>
                    </div>
                {:else}
                    <div class="text-center text-primary mb-5">
                        <p>前点呼アルコール検査の写真無し</p>
                    </div>
                {/if}
                {#if photos.evening_alc}
                    <div class="mb-5">
                        <PhotoCard url={photos.evening_alc} name="後点呼アルコール検査"/>
                    </div>
                {:else}
                    <div class="text-center text-primary mb-5">
                        <p>後点呼アルコール検査の写真無し</p>
                    </div>
                {/if}
                {#if photos.morning_mtr}
                    <div class="mb-5">
                        <PhotoCard url={photos.morning_mtr} name="開始時メーター"/>
                    </div>
                {:else}
                    <div class="text-center text-primary mb-5">
                        <p>開始時メータの写真無し</p>
                    </div>
                {/if}
                {#if photos.evening_mtr}
                    <div class="mb-5">
                        <PhotoCard url={photos.evening_mtr} name="終了時メーター"/>
                    </div>
                {:else}
                    <div class="text-center text-primary mb-5">
                        <p>終了時メータの写真無し</p>
                    </div>
                {/if}
                <div class="mt-20">
                    <p>画像のリンクは30分間有効です</p>
                    <p>リンクが切れた場合は、一度他の日付を閲覧してから再試行して下さい</p>
                </div> 
            {/await}
        </div>
    </div>
</div>
