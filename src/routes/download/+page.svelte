<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import Loading from '$lib/components/Loading.svelte';
    import PhotoCard from "$lib/components/PhotoCard.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").photos}
             */
            const photos = await invoke('load_download');
            return photos;
        } catch {
            goto('/error');
        }

        return {
            morning_alc: '',
            evening_alc: '',
            meter: ''
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
                        <PhotoCard url="{photos.morning_alc}" name="前点呼　アルコール"/>
                    {/if}
                    {#if photos.evening_alc}
                        <PhotoCard url="{photos.evening_alc}" name="後点呼　アルコール"/>
                    {/if}
                    {#if photos.meter}
                        <PhotoCard url="{photos.meter}" name="日報　メーター"/>
                    {/if}
                {/await}
        </div>
    </div>
</div>                