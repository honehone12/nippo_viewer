<script>
    let {q = $bindable()} = $props();
</script>

<select class="select select-primary mb-5" bind:value={q}>
    <option value="" disabled selected>ドキュメントを選択してください</option>
    <option value="calls">点呼</option>
    <option value="reports">日報</option>
</select>