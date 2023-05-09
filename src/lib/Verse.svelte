<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let generate_contextMsg = ''
    let name = ''
  
    async function generate_context() {
        generate_contextMsg = await invoke('generate_context')
    }
  </script>
  
  <div>
    <button on:click="{generate_context}">Generate a Verse</button>
    <h2>{generate_contextMsg}</h2>
  </div>