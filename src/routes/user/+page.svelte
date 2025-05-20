<script>
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import User from "$lib/pages/User.svelte";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Users}
             */
            const users = await invoke('load_users');
            return {
                admin: users.admin,
                users: users.users.map((u) => u.user)
            };
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
    async function selectUser(user) {
        try {
            await invoke('set_query_user', {user});

            goto('/query');
        } catch {
            goto('/error');
        }
    }    
</script>

<User load={load()} {selectUser}/>
