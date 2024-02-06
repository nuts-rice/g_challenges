<script>
import { writable } from "svelte/store";
import { Tab, TabContent } from "carbon-components-svelte";

const tags = writable([]);
const tagsGot = tags.subscribe((value) => {
  console.log(value);
});
</script>

<Tab label="Search tab"/>
<svelte:fragment slot="search content">
<TabContent> Search content </TabContent>
</svelte:fragment>

