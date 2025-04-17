<script lang="ts">
    'use strict';

    interface MorningCall {
        id: string,
        created_at: string,
        caller: string,
        car_number: string,
        method: number,
        using_alc_checker: boolean,
        alc_check: boolean,
        alc_photo: string,
        health_check: boolean,
        car_check: boolean,
        note: string
    }

    interface EveningCall {
        id: string,
        created_at: string,
        caller: string,
        car_number: string,
        method: number,
        using_alc_checker: boolean,
        alc_check: boolean,
        alc_photo: string,
        note: string
    }

    async function load() {

    }
</script>

<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            {#await load()}
                
            {:then calls} 
                
            {/await}
        </div>
    </div>
</div>
