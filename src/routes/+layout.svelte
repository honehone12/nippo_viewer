<script>
    'use strict';

    import "../app.css"
    let {children} = $props();
</script>

<main class="min-w-screen">
    {@render children()}
</main>
