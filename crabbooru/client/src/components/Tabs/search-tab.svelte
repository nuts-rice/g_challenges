<script lang="ts">
import type { Action } from 'svelte/action'
import { onMount } from "svelte";
import { writable } from "svelte/store";
import { Tab, TabContent } from "carbon-components-svelte";
type TagOptions = {
  value: string;
  label: string;
};
const autotag: Action<HTMLTextAreaElement, TagOptions, {'on:changed': (e:)}>
// onMount(() => {
// const tagsGot = tags.subscribe((value) => {
//   console.log(value);
// })})
export function search(site: string, tags: string[] | string = [], {limit = 20, page = 0}:  = {}) {
  const tagsStore = writable(tags);
  const limitStore = writable(limit);
  const pageStore = writable(page);
  const siteStore = writable(site);
  const tagsGot = tagsStore.subscribe((value) => {
    console.log(value);
  });
  const limitGot = limitStore.subscribe((value) => {
    console.log(value);
  });
  const pageGot = pageStore.subscribe((value) => {
    console.log(value);
  });
  const siteGot = siteStore.subscribe((value) => {
    console.log(value);
  });
  return {
    tags: tagsStore,
    limit: limitStore,
    page: pageStore,
    site: siteStore,
    tagsGot,
    limitGot,
    pageGot,
    siteGot
  };
}
</script>

<Tab label="Search tab"/>
<svelte:fragment slot="search content">
<TabContent> Search content </TabContent>
</svelte:fragment>

