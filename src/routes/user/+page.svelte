<script lang="ts">
    'use strict';

    import { goto } from "$app/navigation";
    import { invoke } from "@tauri-apps/api/core";
    import User from "$lib/pages/User.svelte";
    import type { Users } from "$lib/api";

    async function load() {
        try {
            const users = await invoke<Users>('load_users');
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

    async function selectUser(user: string) {
        try {
            await invoke('set_query_user', {user});

            goto('/query');
        } catch {
            goto('/error');
        }
    }    
</script>

<User load={load()} {selectUser}/>
