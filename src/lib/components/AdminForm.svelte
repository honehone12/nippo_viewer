<script>
    'use strict';

    /**
     * @type {{
     *  disabled: boolean,
     *  orgId: string
     * }}
     */
    let {
        disabled,
        orgId = $bindable(),
    } = $props();
</script>

<div>
    <input 
        id="orgId"
        type="text" 
        placeholder="企業IDを英字で入力してください" 
        class="input input-primary"
        {disabled}
        bind:value={orgId}
    >
</div>