<script>
    import AdminForm from "$lib/AdminForm.svelte";

    /**
     * @type {string | undefined}
     */
    let orgId = $state();
    /**
     * @type {string | undefined}
     */
    let adminId = $state();
    /**
     * @type {string | undefined}
     */
    let adminPw = $state();

    /**
     * @returns {boolean}
     */
    function valid() {
        if (!orgId || orgId.length === 0) {
            return false;
        }
        if (!adminId || adminId.length === 0) {
            return false;
        }
        if (!adminPw || adminPw.length === 0) {
            return false;
        }

        return true;
    }

    /**
     * @type {boolean}
     */
    let disabled = $derived(!valid());

    function onclick() {
        if (!valid()) {
            return;
        }

        window.location.href = '/user';
    }
</script>
<div class="hero min-h-screen">
    <div class="hero-content text-center">
        <div class="p-20">
            <div class="text-2xl mb-5">
                <h1 >管理者認証を行います</h1>
            </div>
            <AdminForm 
                bind:orgId 
                bind:adminId
                bind:adminPw
            />
            <div class="mt-5">
                <button 
                    class="btn btn-primary" 
                    {disabled}
                    {onclick}
                >OK</button>    
            </div>
        </div>
    </div>
</div>
