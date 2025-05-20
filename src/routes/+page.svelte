<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import Root from "$lib/pages/Root.svelte";

    let unlistenDone = () => console.warn('auth_done is called without callback set');
    let unlistenFailed = () => console.warn('auth_failed is called without callback set');

    async function load() {
        try {
            /**
             * @type {boolean}
             */
            const exists = await invoke('exists_auth');
            if (exists) {
                goto('/user');
                return true;
            }
        } catch {
            goto('/error');
        }

        return false;
    }

    /**
     * @param {string} url
     */
    function endAuth(url) {
        unlistenDone();
        unlistenFailed();
        goto(url);
    }

    async function startAuth() {
        try {
            unlistenDone = await listen('auth_done', () => endAuth('/auth'));
            unlistenFailed = await listen('auth_failed', () => endAuth('/error'));

            await invoke('start_auth');
        } catch (e) {
            console.error(e);
            goto('/error');
        }
    }
</script>

<Root load={load()} {startAuth}/>
