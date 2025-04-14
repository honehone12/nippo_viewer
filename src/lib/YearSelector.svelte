<script lang="ts">
    let {y = $bindable()}: {y: number} = $props();
</script>

<select class="select select-primary mb-5" bind:value={y}>
    <option value="0" disabled selected>年を選択してください</option>
    <option value="2025">２０２５年</option>
    <option value="2026">２０２６年</option>
</select>
