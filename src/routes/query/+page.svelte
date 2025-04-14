<script>
    'use strict';

    import DocumentSelector from "$lib/DocumentSelector.svelte";
    import MonthSelector from "$lib/MonthSelector.svelte";
    import YearSelector from "$lib/YearSelector.svelte";

    let q = $state('');
    let y = $state(0);
    let m = $state(0);
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            <div class="text-2xl mb-5">
                <h1 >取得するデータの種類を選択してください</h1>
            </div>
            <div>
                <DocumentSelector bind:q/>
            </div>
            <div class="text-2xl mb-5 mt-10">
                <h1 >取得するデータの期間を選択してください</h1>
            </div>
            <div>
                <YearSelector bind:y/>
                <MonthSelector bind:m/>
            </div>
        </div>
    </div>
</div>