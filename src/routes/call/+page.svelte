<script>
    'use strict';
    
    import { goto } from "$app/navigation";
    import Loading from "$lib/components/Loading.svelte";
    import { invoke } from "@tauri-apps/api/core";

    async function load() {
        try {
            /**
             * @type {import("$lib/api").Calls}
             */
            const calls = await invoke('load_calls');
            return calls;  
        } catch {
            goto('/error');
        }

        return {
            morning_calls: [],
            evening_calls: []
        };
    }

    /**
     * @param {number} code 
     */
    function method(code) {
        switch (code) {
            case 1:
                return '対面';
            case 2:
                return '遠隔';
            default:
                return '不明';
        }
    }

    /**
     * @param {boolean} flag 
     */
    function done(flag) {
        return flag ? '有' : '無';
    }

    /**
     * @param {boolean} flag
     */
    function good(flag) {
        return flag ? '良' : '不'
    }
</script>

<div class="flex w-full flex-col lg:flex-row p-20">
    {#await load()}
        <Loading/>
    {:then calls}
        <table class="table table-xs">
            <thead>
                <tr>
                    <th>日時</th>
                    <th>点呼者</th>
                    <th>名前</th>
                    <th>車両番号</th>
                    <th>方法</th>
                    <th>検査機</th>
                    <th>Alc検査</th>
                    <th>写真</th>
                    <th>体調検査</th>
                    <th>車両検査</th>
                </tr>
            </thead>
            <tbody>
                {#each calls.morning_calls as m, i (m.id)}
                    <tr>
                        <td>{m.created_at}</td>
                        <td>{m.caller}</td>
                        <td>{m.name}</td>
                        <td>{m.car_number}</td>
                        <td>{method(m.method)}</td>
                        <td>{done(m.using_alc_checker)}</td>
                        <td>{good(m.alc_check)}</td>
                        <td>{done(!!m.alc_photo)}</td>
                        <td>{good(m.health_check)}</td>
                        <td>{good(m.car_check)}</td>
                    </tr>
                    <tr>
                        <th>備考</th>
                        <td colspan="10">{m.note}</td>
                    </tr>
                {/each}
            </tbody>
            
        </table> 
        <div class="divider lg:divider-horizontal divider-primary"></div>
        <table class="table table-xs">
            <thead>
                <tr>
                    <th>日時</th>
                    <th>点呼者</th>
                    <th>名前</th>
                    <th>車両番号</th>
                    <th>方法</th>
                    <th>検査機</th>
                    <th>Alc検査</th>
                    <th>写真</th>
                </tr>
            </thead>
            <tbody>
                {#each calls.evening_calls as e, i (e.id)}
                    <tr>
                        <td>{e.created_at}</td>
                        <td>{e.caller}</td>
                        <td>{e.name}</td>
                        <td>{e.car_number}</td>
                        <td>{method(e.method)}</td>
                        <td>{done(e.using_alc_checker)}</td>
                        <td>{good(e.alc_check)}</td>
                        <td>{done(!!e.alc_photo)}</td>
                    </tr>
                    <tr>
                        <th>備考</th>
                        <td colspan="8">{e.note}</td>
                    </tr>
                {/each}
            </tbody>
        </table> 
    {/await}
</div>
