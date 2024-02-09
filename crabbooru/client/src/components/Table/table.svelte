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
    import type {DanbooruItem} from '../../api/image_api';

    import type {TestbooruItem} from '../../api/image_api';
    import {getTestbooruCall} from '../../api/image_api';

    import {getDanbooruCall} from '../../api/image_api';
    import {getDanbooruImage} from '../../api/image_api';
  import { Image } from '@unpic/svelte';

    export let columnCount = "4";
    export let rowCount = "4";
    export let itemCount = "";
    export let border = "";
    export let placeholderImgs = "false";
    let placeHolderItemCount = 10;
//    export const test_item = writable<Image>(); 
    export let id = "5942";    
    export let tags : string[] = [];
    tags = ["touhou", "reimu_hakurei"];
    export const test_item = writable<TestbooruItem>();
    export const dan_item = writable<DanbooruItem>();


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

        let test_item = await getTestbooruCall(tags,2, "20" )
        let dan_item = await getDanbooruCall(tags,2, "20" )

        console.log("svelte: onmount: test_call: " + test_item);
        console.log("svelte: onmount: dan_call: " + dan_item);

        }
        

     );

    </script>

<!-- <button on:click={getTestbooruImage}> Load demo images</button> -->
<!-- <ImageApi bind:this={api_call}/> -->
    <ImageGrid columns={columnCount} rows={rowCount} {border}>
        <!-- {#each $item as img_item (img_item)} -->
       <div> 
            <!-- <Cell placeholder= ...  -->
            Test image lol            
            {test_item} 
            <!--  /> -->
        </div>
        <div>
        <slot/>
        <!-- {/each} -->
        </ImageGrid>
<Pagination totalItems={placeHolderItemCount} pageSizeInputDisabled/>
