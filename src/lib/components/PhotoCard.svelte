<script>
    'use strict';

    /**
     * @type {{url: string, name: string}}
     */
    let {url, name} = $props();
</script>

<div class="card card-side bg-base-300 mb-5">
    <figure>
        <img src="{url}" class="h-120" alt="{name}"/>
    </figure>
    <div class="card-body place-content-center px-10">
        <h2 class="card-title text-2xl mb-10">{name}</h2>
        <div class="card-actions justify-center">
            <a href="{url}" download>
                <button class="btn btn-primary">ダウンロード</button>
            </a>
        </div>
    </div>
</div>