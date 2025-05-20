<script>
    'use strict';

    import {goto} from "$app/navigation";
    import {invoke} from "@tauri-apps/api/core";
    import Invite from "$lib/pages/Invite.svelte";
    
    async function load() {
        try {
            /**
             * @type {import("$lib/api").Users}
             */
            const users = await invoke('load_users');
            if (users.admin) {
                return {
                    admin: users.admin,
                    users: users.users.filter((u) => u.invitable).map((u) => u.user)
                }
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
    async function invite(user) {
        try {
            /**
             * @type {string}
             */
            const invited = await invoke('invite', {user});
            return invited;
        } catch {
            goto('/error');
        }

        return '';
    }
</script>

<Invite load={load()} {invite}/>