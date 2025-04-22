<script>
    'use strict';

    /**
     * @type {{url: string, name: string}}
     */
    let {url, name} = $props();
</script>

<div class="card card-side bg-base-200">
    <figure>
        <img src="{url}" class="max-h-100 max-w-100" alt="{name}"/>
    </figure>
    <div class="card-body place-content-center px-10">
        <h2 class="card-title text-xl mb-5">{name}</h2>
        <div class="card-actions justify-center">
            <a href="{url}" download>
                <button class="btn btn-primary">ダウンロード</button>
            </a>
        </div>
    </div>
</div>