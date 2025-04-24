<script>
    import { page } from "$app/state";

    'use strict';

    function onload() {
        const url = page.url;

        console.log(url);
    }

    onload();
</script>