<script lang="ts">
    'use strict';

    import { goto } from "$app/navigation";
    import Loading from "$lib/components/Loading.svelte";
    import UserSelector from "$lib/components/UserSelector.svelte";
    import { invoke } from "@tauri-apps/api/core";

    async function load() {
        try {
            const json = await invoke('load_users');
            console.log(json);
            return json;
        } catch (e) {
            console.error(e);
            goto('/error');
        }

    }

    function onclick() {

    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                <Loading/>
            {:then res} 
                <div class="text-2xl mb-5">
                    <h1 >取得するユーザーを選択してください</h1>
                </div>
                <div>
                    <UserSelector/>
                </div>
                <div class="mt-10">
                    <button 
                        class="btn btn-primary" 
                        {onclick}
                    >OK</button>    
                </div>
            {/await}

            
        </div>
    </div>
</div>