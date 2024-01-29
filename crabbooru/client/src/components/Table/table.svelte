<svelte:options tag = "cobalt-table"/>
<script>
    import {invoke} from '@tauri-apps/api/tauri';
    import ImageGrid from "./image-grid.svelte";
    import Cell from "./cell.svelte";
    export let columnCount = "4";
    export let rowCount = "4";
    export let itemCount = "";
    export let border = "";
    export let placeholderImgs = "false";
    async function getImages() {
        let images = await invoke('get_images', {itemCount});
    }
    </script>


    <ImageGrid columns= {columnCount} rows={rowCount} {border}>
        {#each {length: parseInt(itemCount, 0)} as _, i}
        <Cell  />
        {/each}
        </ImageGrid>
