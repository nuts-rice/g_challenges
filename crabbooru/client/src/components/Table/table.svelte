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
    import type {SingleTestbooruResponse}from '../../api/image_api';

    import type {TestbooruItem} from '../../api/image_api';
    import {getSingleTestbooruImg} from '../../api/image_api';
    import {getTestbooruImage} from '../../api/image_api';
    import {getTestbooruCall} from '../../api/image_api';
import {getTestbooruCallId} from '../../api/image_api';
import BooruResult from '../../api/image_api';


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
    export const test_item = "";

    export const test_items = writable<TestbooruItem[]>();
    export const dan_items = writable<DanbooruItem[]>();
    export let test_response = writable<TestbooruItem>();

function setup(event: Event) {
const imgInput = document.getElementById('image');
imgInput.addEventListener('change', function(event) {
  const target = event.target.files[0];
  const file: File = (target.files as FileList)[0];
  const reader = new FileReader();
  reader.onload = (e) => {
    const img = document.getElementById('img') as HTMLImageElement;
    img.src = e.target.result as string;
  };
  reader.readAsDataURL(file);
});
}


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

        let test_items = await getTestbooruCall(tags,2, 20 )
        let test_item = await getTestbooruCallId(parseInt(id));
  
        
        
  
        // let dan_items = await getDanbooruCall(tags,2, "20" )

        console.log("svelte: onmount: test_call: " + test_items);
        console.log("svelte: onmount: test_img: " + test_item);

        console.log("svelte: onmount: dan_call: " + dan_items);

        }
        

     );

    </script>

<!-- <button on:click={getTestbooruImage}> Load demo images</button> -->
<!-- <ImageApi bind:this={api_call}/> -->
    <ImageGrid columns={columnCount} rows={rowCount} {border}>
        <!-- {#each $test_items as test_img, i} -->
       <div> 
            <!-- <Cell placeholder= ...  -->
    <input type="file"
    name="booru-img"
    id="booru-img"
    value=""  
    >  
    <br>  
    <canvas id="preview"></canvas>
            Test image lol            
            {test_items}
            <img src={test_item} alt="booru img" />
        
            <!--  /> -->
        </div>
        <!-- <div></div>
            Dan image lol
            
        {dan_items}
         -->
        <slot/>
        <!-- {/each} -->
        </ImageGrid>
<Pagination totalItems={placeHolderItemCount} pageSizeInputDisabled/>
