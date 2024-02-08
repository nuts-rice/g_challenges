<svelte:options tag = "cobalt-table"/>
<script lang="ts">
import {setContext} from 'svelte';
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
    import type {TestbooruItem} from '../../api/image_api';
    import {getTestbooruCall} from '../../api/image_api';
    import {getTestbooruImage} from '../../api/image_api';
  import { Image } from '@unpic/svelte';

    export let columnCount = "4";
    export let rowCount = "4";
    export let itemCount = "";
    export let border = "";
    export let placeholderImgs = "false";
    let placeHolderItemCount = 10;
    export const test_item = writable<Image>(); 
    export let id = "5942";    
    export const item = writable<TestbooruItem>();

    // setContext('items', items);
//    export const paginatedItems = derived([items, currentPage, perPage], ([$items, $currentPage, $perPage]) => {
//        return paginate({items: $items, perPage: $perPage, currentPage: $currentPage});
//    });
    // async function getImages() {
    //     console.log("svelte: getimages: " + $items )
    //     items = await invoke('get_images_cmd', {id: 5942});
    // }
    // onMount(async () => {
    //   if (placeholderImgs) {
    //       getImages();
    //   }
    // })
     onMount(async () => {

        let item = await getTestbooruImage(id, );
        }
        

     );

    </script>

<!-- <button on:click={console.log("api call: ")}> Load demo images</button> -->
<!-- <ImageApi bind:this={api_call}/> -->
    <ImageGrid columns={columnCount} rows={rowCount} {border}>
        <!-- {#each $item as img_item (img_item)} -->
       <div> 
            <!-- <Cell placeholder= ...  -->
            {item} 
            <!--  /> -->
        </div>
        <div>
            Test image lol            
            <img src = "localhost:testimg.jpg" alt="placeholder" />
        <slot/>
        <!-- {/each} -->
        </ImageGrid>
<Pagination totalItems={placeHolderItemCount} pageSizeInputDisabled/>
