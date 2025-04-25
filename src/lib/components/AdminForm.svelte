<script lang="ts">
    'use strict';

    interface Props {
        disabled: boolean,
        orgId: string,
    }

    let {
        disabled,
        orgId = $bindable(),
    }: Props = $props();
</script>

<div>
    <input 
        id="orgId"
        type="text" 
        placeholder="企業IDを英字で入力してください" 
        class="input input-primary mb-5"
        {disabled}
        bind:value={orgId}
    >
</div>