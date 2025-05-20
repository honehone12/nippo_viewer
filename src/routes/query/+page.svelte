<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import DocumentSelector from "$lib/components/DocumentSelector.svelte";
    import MonthSelector from "$lib/components/MonthSelector.svelte";
    import YearSelector from "$lib/components/YearSelector.svelte";
    import { invoke } from "@tauri-apps/api/core";

    let q = $state('');
    let y = $state('');
    let m = $state('');
    let submitting = $state(false);

    function valid() {
        if (submitting) {
            return false;
        }

        return !!q && !!y && !!m
    }

    async function onclick() {
        if (!valid()) {
            return;
        }

        submitting = true;

        try {
            await invoke('set_query_ym', {y, m});

            switch (q) {
                case 'reports':
                    goto('/report');
                    break;
                case 'calls':
                    goto('/call');
                    break;
                default:
                    goto('/error');
                    break;
            }
        } catch {
            goto('/error');
        }
    }

    let ready = $derived(valid());
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            <div class="text-2xl text-primary mb-5">
                <h1 >取得するデータの種類を選択してください</h1>
            </div>
            <div>
                <DocumentSelector bind:q/>
            </div>
            <div class="text-2xl text-primary mb-5 mt-10">
                <h1 >取得するデータの期間を選択してください</h1>
            </div>
            <div>
                <YearSelector bind:y/>
                <MonthSelector bind:m/>
            </div>
            <div class="mt-10">
                <button 
                    class="btn btn-primary" 
                    disabled={!ready}
                    {onclick}
                >OK</button>    
            </div>
        </div>
    </div>
</div>