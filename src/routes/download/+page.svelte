<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Loading from '$lib/components/DataLoading.svelte';
    import PhotoCard from "$lib/components/PhotoCard.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Photos}
             */
            const photos = await invoke('load_download');
            return photos;
        } catch {
            goto('/error');
        }

        return {
            morning_alc: '',
            evening_alc: '',
            morning_mtr: '',
            evening_mtr: ''
        };
    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                <Loading/>
            {:then photos}
                <div class="mb-15">
                    <p>リンクは30分間有効です</p>
                    <p>リンクが切れた場合は、一度他の日付を閲覧してから再試行して下さい</p>
                </div> 
                {#if photos.morning_alc}
                    <div class="mb-5">
                        <PhotoCard url="{photos.morning_alc}" name="前点呼アルコール検査"/>
                    </div>
                {:else}
                    <div class="text-center mb-5">
                        <p>前点呼アルコール検査の写真無し</p>
                    </div>
                {/if}
                {#if photos.evening_alc}
                    <div class="mb-5">
                        <PhotoCard url="{photos.evening_alc}" name="後点呼アルコール検査"/>
                    </div>
                {:else}
                    <div class="text-center mb-5">
                        <p>後点呼アルコール検査の写真無し</p>
                    </div>
                {/if}
                {#if photos.morning_mtr}
                    <div class="mb-5">
                        <PhotoCard url="{photos.morning_mtr}" name="開始時メーター"/>
                    </div>
                {:else}
                    <div class="text-center mb-5">
                        <p>開始時メータの写真無し</p>
                    </div>
                {/if}
                {#if photos.evening_mtr}
                    <div class="mb-5">
                        <PhotoCard url="{photos.evening_mtr}" name="終了時メーター"/>
                    </div>
                {:else}
                    <div class="text-center mb-5">
                        <p>終了時メータの写真無し</p>
                    </div>
                {/if}
            {/await}
        </div>
    </div>
</div>                