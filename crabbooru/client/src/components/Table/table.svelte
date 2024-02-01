<svelte:options tag = "cobalt-table"/>
<script>
import {Grid, Row, Column}    from "carbon-components-svelte";
import {Pagination} from "carbon-components-svelte";
import SvelteTable from 'svelte-table';
    import {paginate, LightPaginationNav} from 'svelte-paginate';
    import {writable, derived} from 'svelte/store';
    import {invoke} from '@tauri-apps/api/tauri';
    import {onMount} from 'svelte';
    import ImageGrid from "./image-grid.svelte";
    import currentPage from "./image-grid.svelte";
    import perPage from "./image-grid.svelte";
    import Cell from "./cell.svelte";
    export let columnCount = "4";
    export let rowCount = "4";
    export let itemCount = "";
    export let border = "";
    export let placeholderImgs = "false";
    const items = writable([]);
//    export const paginatedItems = derived([items, currentPage, perPage], ([$items, $currentPage, $perPage]) => {
//        return paginate({items: $items, perPage: $perPage, currentPage: $currentPage});
//    });
    async function getImages() {
        console.log("svelte: getimages: " )
        let items = await invoke('get_images', {id: "1"});
    }
    // onMount(() => {
    //   getImages();
    // });

    </script>



    <ImageGrid columns={columnCount} rows={rowCount} {border}>
        {#each $items as item, i}
        <Cell {item}/>
        {/each}
        </ImageGrid>
<Pagination totalItems={itemCount} pageSizeInputDisabled/>
