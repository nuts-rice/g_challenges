<script lang="ts">
import {invoke} from '@tauri-apps/api/tauri';
//let sourceRows = SourceRow[];
let sources : [];
let selected = false;
async function getSources() {
  sources = await invoke('get_sources', {sources} );
}

</script>

<div>
{#each sources as source}
<label>
<input type="checkbox" checked={selected}/>
{getSources}
</label>
{/each}
</div>


