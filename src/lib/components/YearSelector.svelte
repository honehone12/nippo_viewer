<script>
    'use strict';

    /**
     * @type {{y: string}}
     */
    let {y = $bindable()} = $props();
</script>

<select id="ySelect" class="select select-primary mb-5" bind:value={y}>
    <option value="" disabled selected>年を選択してください</option>
    <option value="2025">２０２５年</option>
    <option value="2026">２０２６年</option>
</select>
