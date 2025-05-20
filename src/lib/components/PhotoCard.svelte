<script>
    'use strict';

    /**
     * @type {{url: string, name: string}}
     */
    let {url, name} = $props();
</script>

<div class="card card-side bg-base-200">
    <figure>
        <img src="{url}" class="w-100" alt="{name}"/>
    </figure>
    <div class="card-body place-content-center px-10">
        <div class="text-center">
            <h2 class="text-xl text-primary mb-5">{name}</h2>
        </div>
        <div class="justify-center">
            <a href="{url}">
                <button class="btn btn-primary">ダウンロード</button>
            </a>
        </div>
    </div>
</div>