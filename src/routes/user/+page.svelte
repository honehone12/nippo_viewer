<script>
    'use strict';
    
    import UserSelector from "$lib/UserSelector.svelte";

    let {users} = $props();

    function onclick() {

    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            <div class="text-2xl mb-5">
                <h1 >取得するユーザーを選択してください</h1>
            </div>
            <div>
                <UserSelector/>
            </div>
            <div class="mt-10">
                <button 
                    class="btn btn-primary" 
                    {onclick}
                >OK</button>    
            </div>
        </div>
    </div>
</div>