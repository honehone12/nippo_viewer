<script lang="ts">
    'use strict';

    interface Props {
        disabled: boolean,
        orgId: string,
        adminId: string,
        adminPw: string
    }

    let {
        disabled,
        orgId = $bindable(),
        adminId = $bindable(),
        adminPw = $bindable()
    }: Props = $props();
</script>

<div>
    <input 
        type="text" 
        placeholder="企業IDを英字で入力してください" 
        class="input input-primary mb-5"
        {disabled}
        bind:value={orgId}
    >
    <input 
        type="text" 
        placeholder="管理者IDを英字で入力してください" 
        class="input input-primary mb-5"
        {disabled}
        bind:value={adminId}
    >
    <input 
        type="password" 
        placeholder="管理者パスワードを英字で入力してください" 
        class="input input-primary mb-5"
        {disabled}
        bind:value={adminPw}
    >
</div>