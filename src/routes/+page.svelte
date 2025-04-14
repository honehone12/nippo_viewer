<script>
    import AdminForm from "$lib/AdminForm.svelte";

    let orgId = $state('');
    let adminId = $state('');
    let adminPw = $state('');
    let loading = $state(false);

    function valid() {
        if (loading) {
            return false;
        }

        if (orgId.length === 0) {
            return false;
        }
        if (adminId.length === 0) {
            return false;
        }
        if (adminPw.length === 0) {
            return false;
        }

        return true;
    }

    let disabled = $derived(!valid());

    function onclick() {
        if (!valid()) {
            return;
        }
        loading = true;

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
