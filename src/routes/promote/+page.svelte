<script>
    'use strict';

    import {goto} from "$app/navigation";
    import {invoke} from "@tauri-apps/api/core";
    import Promote from "$lib/pages/Promote.svelte";
    
    async function load() {
        try {
            /**
             * @type {import("$lib/api").Users}
             */
            const users = await invoke('load_users');

            if (users.admin) {
                return {
                    admin: users.admin,
                    users: users.users.filter((u) => u.promotable).map((u) => u.user)
                };
            }
            
            goto('/error');
        } catch {
            goto('/error');
        }

        return {
            admin: false,
            users: []
        };
    }

    /**
     * @param {string} user
     */
    async function promote(user) {
        try {
            await invoke('promote', {user});
        } catch {
            goto('/error');
        }
    }
</script>

<Promote load={load()} {promote} />
