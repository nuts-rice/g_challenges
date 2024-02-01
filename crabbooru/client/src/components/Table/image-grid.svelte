<svelte:options tag="cobalt-grid"/>
<script>
import {Grid, Row, Column}    from "carbon-components-svelte";
import {paginate, LightPaginationNav} from 'svelte-paginate'; 
import {writable, derived} from 'svelte/store';
import {invoke} from '@tauri-apps/api/tauri';
import {onMount} from 'svelte';
import itemCount from "./table.svelte";
// export let src="";
// export let alt="";
// export let is_loaded = false;
// export let imgs = [...];  
export let columns = "4";
export let rows = "4";
export let border = "1px solid #000000";
let colInt = parseInt(columns, 10);
let rowInt = parseInt(rows, 10);
export const currentPage = writable(1) ;
export let perPage = 10;
export const totalPages = derived(itemCount, ($itemCount) => Math.ceil($itemCount / perPage));

// export let trimmedRows;
// $: paginatedImgs = paginate({imgs, perPage, currentPage})

// $: totalRows = rows.length
// $: currentPage = 0
// $: totalPages = Math.ceil(totalRows / perPage)
// $: start = currentPage * perPage
// $: end = currentPage === totalPages - 1 ? totalRows - 1 : start + perPage - 1 ;
// $: trimmedRows = rows.slice(start, end + 1)
// $: totalRows, currentPage = 0
// $: currentPage, start, end

</script>
<!-- {#if totalRows && totalRows > perPage}
<div class = 'image-grid'>
   <button on:click={() => currentPage -= 1}
    >  -->

    
<div
style="
grid-template-columns: repeat({colInt}, auto);
grid-template-rows: repeat({rowInt}, 4fr); 
border: {border};
"
>
<slot/>
</div>   
<!-- <ul class = "imgs">    
    {#each paginatedImgs as img}
    <li class = "img">
        {img}
        </li>
        {/each}
        </ul> -->

<style>
    div {
        font-family: Arial, Helvetica, sans-serif;
        display: grid;
        grid-column-gap: 10px;
        grid-row-gap: 5px;
        grid-auto-flow: column;
        border: 1px solid black;

    }
    img {
        width: 100%;        
        height: 100%;
        object-fit: cover;
    }
    img.is_loaded {
        opacity: 1;
        transition: opacity 0.5s ease-in;
    }
</style>    
<!-- <img {src} {alt} class:is_loaded> -->
