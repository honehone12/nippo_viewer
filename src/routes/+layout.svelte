<script>
    'use strict';
    
    import "../app.css";
    import Navbar from "$lib/components/layout/Navbar.svelte";

    let {children} = $props();
</script>

<Navbar/>
<main class="w-full">
    {@render children()}
</main>
