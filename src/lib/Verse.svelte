<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let generate_contextMsg = ''
  
    async function generate_context() {
        generate_contextMsg = await invoke('generate_context')
    }
</script>
  
<div class="container">

    <button type="button" class="btn bg-gradient-to-br variant-gradient-warning-error" on:click="{generate_context}">Generate a verse</button>
    <h3 class="h3"><span class="bg-gradient-to-br from-red-500 to-yellow-500 bg-clip-text text-transparent box-decoration-clone">{generate_contextMsg}</span></h3>

</div>

<style>
    .container {
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
    }
    h3 {
    display: block;
    margin: 0 auto;
    height: 1.5rem;
    text-align: center;
    margin-top: 20px;
    }
</style>
   