<script lang="ts">
    interface Props {
        orgId: string,
        adminId: string,
        adminPw: string
    }

    let {
        orgId = $bindable(),
        adminId = $bindable(),
        adminPw = $bindable()
    }: Props = $props();
</script>

<div>
    <input 
        type="text" 
        placeholder="企業IDを入力してください" 
        class="input input-primary mb-5"
        bind:value={orgId}
    >
    <input 
        type="text" 
        placeholder="管理者IDを入力してください" 
        class="input input-primary mb-5"
        bind:value={adminId}
    >
    <input 
        type="text" 
        placeholder="管理者パスワードを入力してください" 
        class="input input-primary mb-5"
        bind:value={adminPw}
    >
</div>