<script lang="ts">
import {invoke} from '@tauri-apps/api/tauri';
//let sourceRows = SourceRow[];
let sources = ["danbooru", "gelbooru"];
let is_selected = false;
async function getSources() {
  sources = await invoke('get_sources', {sources} );
}

</script>
<h1> Sources </h1>
<div>
{#each sources as source}
<label>
<div>
{source} <input type="checkbox" checked={is_selected}/>
</div>
</label>
{/each}
</div>


