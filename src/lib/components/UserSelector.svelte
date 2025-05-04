<script>
    'use strict';

    /**
     * @type {{
     *  admin: boolean
     *  users: import("$lib/api").User[], 
     *  user: string
     * }}
     */
    let {
        admin,
        users, 
        user = $bindable()
    } = $props()
</script>

<select 
    id="uSelect" 
    class="select {admin ? "select-secondary": "select-primary"} mb-5" 
    bind:value={user}
>
    <option value="" disabled selected>ユーザーを選択してください</option>
    {#each users as u}
        <option value="{u.id}">{u.name}</option>
    {/each}
</select>
