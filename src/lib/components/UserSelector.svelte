<script lang="ts">
    'use strict';

    interface User {
        id: string,
        name: string
    }

    let {
        users, 
        user = $bindable()
    }: {users: Array<User>, user: string} = $props()
</script>

<select class="select select-primary mb-5" bind:value={user}>
    <option value="" disabled selected>ユーザーを選択してください</option>
    {#each users as u}
        <option value="{u.id}">{u.name}</option>
    {/each}
</select>
