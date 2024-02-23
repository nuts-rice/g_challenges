<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher, onMount } from "svelte";
  import { Checkbox, Dropdown, Search, ButtonSet, Button, Tag } from "carbon-components-svelte";
  import BooruResult from "../../api/image_api"
  import BooruSite from "../../api/image_api"
  import { readCSV } from "../../utils/file";
  //    import SearchBar from './SearchBar';
  let booru_sites = ["Testbooru", "Safebooru", "Danbooru"]
  let isValid = false;
  let errorMsg = "";
  export let search;
  let selectedBooruId = 0;
  let isDropdownOpen = false;
  let searchQuery = [""];
  let searching = false;
  let suggestions = undefined;
  let currentSearch = undefined;
  let checked = false;
  let booruGroup = booru_sites.slice(0, 2);
  const handleBooruDropdown = () => {
    isDropdownOpen = !isDropdownOpen;
  };
  const handleBooruSelect = async (id: number) => {
    selectedBooruId = id;
    isDropdownOpen = false;
    console.log("selected booru id: ", selectedBooruId);
    let booru_site: BooruSite =  selectedBooruId
      
      
  };

  const setQuery = async (_searchQuery: string) => {
    searchQuery = _searchQuery;
    searching = true;
    let _currentSearch = {};
    const data = await search(_searchQuery);
    if (currentSearch === _currentSearch) {
      suggestions = results;
      searching = false;
    }
  };


  function validateSearchQuery(input: any) {
    function onInput() {
      const isValid = input.checkValidity();
      const errorMsg = isValid ? "" : getErrorMsg(input.validity);
      input.dispatchEvent(
        new CustomEvent("validate", { detail: { isValid, errorMsg } }),
      );
    }

    function getErrorMsg(validity: any) {
      if (validity.valueMissing) {
        return "Please enter a search query";
      } else if (validity.tooShort) {
        return "Search query must be at least 3 characters";
      } else if (validity.tooLong) {
        return "Search query must be at most 50 characters";
      } else {
        return "";
      }
    }
    input.addEventListener("input", onInput);
    return {
      destroy() {
        input.removeEventListener("input", onInput);
      },
    };
  }

  const dispatch = createEventDispatcher<{
    loaded: null;
    clicked: string;
  }>();

  onMount(() => {
    dispatch("loaded");
    console.log("searchbar mounted query: ", searchQuery);
  });

  function matchBooru(selected: number)  {
  //  if 
 //     return booru
        
  };
  function onInput(query: string) {
    console.log("searchQuery: ", searchQuery);
    let search_result = new BooruResult (selectedBooruId, 1, searchQuery, [])
    let tags = query.split(" ");
//    search_result.booru_call()
    dispatch("clicked", searchQuery.toString());
  }
</script>

<input
  required
  minlength="3"
  class:invalid={!isValid}
  use:validateSearchQuery
  on:validate={(event) => {
    isValid = event.detail.isValid;
    errorMsg = event.detail.errorMsg;
  }}
/>
{errorMsg}
validate search query test
<section>
  <!-- TODO: this should be checkbox  --->
  {#each booru_sites as booru}
    <Checkbox bind:booruGroup labelText={booru} {booru}/>
{/each}
<!--  <Dropdown bind:checked 
  size="sm"  
  labelText="Booru"
  selectedId = "0"
  on:select={handleBooruSelect}
  items={[
    {id: "0", text: "Testbooru"},
    {id: "1", text: "Safebooru"},
    {id: "2", text: "Danbooru"},
    ]}
    let:item
    let:index
  >
  </Dropdown> ---> 
  <div>
    <Search bind:searchQuery />
    SearchQuery test
  </div>
</section>
<div>
  <ButtonSet>
    <Button
      size="small"
      disabled={searchQuery === "Tags..."}
      on:click={() => onInput(searchQuery)}
    >
      Search boorus
    </Button>
    <Button
      kind="ghost"
      size="small"
      disabled={searchQuery.length === 0}
      on:click={() => (searchQuery = "")}
    >
      Clear query
    </Button>
  </ButtonSet>
</div>
<section>
  <ul>
    <div>Query: <Tag size="sm">{searchQuery}</Tag></div>
  </ul>
</section>

<style>
  div {
    margin-top: var(--cds-spacing-05);
  }
</style>
