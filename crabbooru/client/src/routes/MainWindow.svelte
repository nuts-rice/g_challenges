<script lang="ts">
import {invoke} from '@tauri-apps/api/tauri';
// import ImageGrid from '../components/image-grid.svelte'; 
let windows = ["sources", "sites", "gallery", "about", "settings"];
//let images = [];
let is_selected = false;
async function getMainWindow() {
  window = await invoke('get_main_window', {windows} );
}



</script>
<h1> Main Window </h1>
<ul>
{#each windows as window}
<button>
<label>
<div>
{window}
</div>
</label>
</button>
{/each}
</ul>
<!-- {#if images}
{#each images as image}
<ImageGrid {rows} {perPage} />
{/each}
{/if} -->