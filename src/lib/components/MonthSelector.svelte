<script>
    'use strict';
    
    /**
     * @type {{m: string}}
     */
    let {m = $bindable()} = $props();
</script>

<select id="mSelect" class="select select-primary mb-5" bind:value={m}>
    <option value="" disabled selected>月を選択してください</option>
    <option value="1">１月</option>
    <option value="2">２月</option>
    <option value="3">３月</option>
    <option value="4">４月</option>
    <option value="5">５月</option>
    <option value="6">６月</option>
    <option value="7">７月</option>
    <option value="8">８月</option>
    <option value="9">９月</option>
    <option value="10">１０月</option>
    <option value="11">１１月</option>
    <option value="12">１２月</option>
</select>
