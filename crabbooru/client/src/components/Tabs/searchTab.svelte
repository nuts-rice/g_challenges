<script lang="ts">
import type { Action } from 'svelte/action'
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { getTestbooruCall } from '../../api/image_api';
import type { TestbooruItem } from '../../api/image_api';
import  SearchBar from '../Search/searchBar.svelte';
import { Tab, TabContent } from "carbon-components-svelte";
// let searchQuery = "";
// let tagOptions  = writable([]); 
// let testTagOptions = writable([]);
// type TagOptions = {
//   value: string;
//   label: string;
// };
onMount(async () => {
  const searchBar = new SearchBar({
    target: document.body,
    props: {
      search: "test"
    }
  }); 
})
// onMount(async () => {
//   const response  = await getTestbooruCall(searchQuery, 1, 10); 
// //  searchQuery.subscribe(async (currentQuery) => {
// //   if (currentQuery) {
// //     const response = await getTestbooruCall(currentQuery, 1, 10);
// //     console.log(response);
// //     // tagOptions.set(response?.item_list); 
// //   }
// //  })
// // })
// function handleInput(event: InputEvent) {
//   const input = event.target as HTMLInputElement;
//   console.log(input.value);
//   // searchQuery.set(input.value.split(" "));
// }
// const autotag: Action<HTMLTextAreaElement, TagOptions, {'on:changed': (e: CustomEvent<{value: string}>) => void}> = (node, options) => {
//   const { value, label } = options;
//   const tags = writable([]);
//   const tagsGot = tags.subscribe((value) => {
//     console.log(value);
//   });
//   const onchanged = (e: CustomEvent<{value: string}>) => {
//     console.log(e.detail.value);
//     tags.update((value) => {
//       return [...value, e.detail.value];
//     });
//   };
//   return {
//     on: {
//       changed: onchanged
//     }
//   };
// };


// onMount(() => {
// const tagsGot = tags.subscribe((value) => {
//   console.log(value);
// })})
</script>

<Tab label="Search tab"> 

<!-- <svelte:fragment slot="search content"> -->
  <TabContent>
    <!-- <input type="text" on:input={handleInput} placeholder="Search..."/> -->
    <ul>
      {SearchBar}
      <!-- {#each $tagOptions as tag (tag.value)}
      <li> {tag.label} </li>
      {/each} -->
      </ul>
  </TabContent>
</Tab>
<!-- </svelte:fragment> -->
<style>
.tab-content {
  overflow: scroll; 
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}


</style>