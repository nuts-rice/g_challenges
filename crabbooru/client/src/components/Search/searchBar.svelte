<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher, onMount } from "svelte";
  import { Checkbox, Dropdown, Search, ButtonSet, Button, Tag } from "carbon-components-svelte";
  import AutoComplete from "simple-svelte-autocomplete";
  import BooruResult from "../../api/image_api"
  import  booru_call  from "../../api/image_api"
  import BooruSite from "../../api/image_api"
  import { readCSV } from "../../utils/file";
  //    import SearchBar from './SearchBar';
  let booru_sites = ["Testbooru", "Safebooru", "Danbooru"]
  let isValid = false;
  let errorMsg = "";
  export let search;
  let selectedBooruId = 0;
  let isDropdownOpen = false;
  let value = "";
  let searching = false;
  let checked = false;
  let tag_suggestions : string[] = [];
  let selectedTag: string;
  let filtered_suggestions : string[] = []; 
  export let suggestionLabel;
  export let highlighted;
  let group = booru_sites.slice(0, 2);
  const handleBooruDropdown = () => {
    isDropdownOpen = !isDropdownOpen;
  };
//  const handleBooruSelect = async (id: number) => {
//    selectedBooruId = id;
//    isDropdownOpen = false;
//    console.log("selected booru id: ", selectedBooruId);
//    let booru_site:  =  selectedBooruId
    
      
//  };
  

  async function filterSuggestions (filtered_suggestions) {
    let storageArr: string[] = []
    if (value) {
      tag_suggestions.forEach((tag) => {
        if (tag.toLowerCase().startsWith(value.toLowerCase())) {
          storageArr.push(tag);
        }
      });
      filtered_suggestions = storageArr;
    } else {
      filtered_suggestions = tag_suggestions;

    }
  }
  const setQuery = async (value: string) => {
    let searchQuery =value;
    searching = true;
    let _currentSearch = {};
    
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

async function handleAutoComplete() {
    let tag_suggestions = await invoke <string[]>("auto_tags_cmd", {input: value});
    console.log("autocomplete");
  }

  function getBooruSelection() {
    return booru_sites.filter((_, index) => booruGroup[index]);
  }

  const dispatch = createEventDispatcher<{
    loaded: null;
    clicked: string;
  }>();

  onMount(() => {
    dispatch("loaded");
    console.log("searchbar mounted query: ", value);
  });

  function matchBooru(selected: number)  {
  //  if 
 //     return booru
        
  };
  function onInput(query: string) {
    value = query;
    searching = true
    let selected_boorus_json = JSON.stringify(group); 
    let parsed_query = query.split(" ");
    let parsed_boorus = selected_boorus_json.split(",");

    console.log("selected boorus: ", group);
    console.log("parsed query:  ", parsed_query);
    let search_query = new BooruResult (group, 1, parsed_query, [], );
      let cmd_result =  search_query.booru_call(parsed_query, 1, 10);
//    search_result.booru_call()
//    dispatch("clicked", searchQuery.toString());
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
  {#each booru_sites as value}
    <Checkbox bind:group labelText={value} {value}/>
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
    Autocomplete test
    <AutoComplete searchFunction="{handleAutoComplete}" delay="200" items="{tag_suggestions}" bind:selectedItem="{selectedTag}" />
    <Search bind:value />
    SearchQuery test
  </div>
</section>
<div>
  <ButtonSet>
    <Button
      size="small"
      disabled={value === "Tags..."}
      on:click={() => onInput(value)}
    >
      Search boorus
    </Button>
    <Button
      kind="ghost"
      size="small"
      disabled={value.length === 0}
      on:click={() => (value = "")}
    >
      Clear query
    </Button>
  </ButtonSet>
</div>
<section>
  <ul>
    <div>Query: <Tag size="sm">{value}</Tag></div>
  </ul>
</section>

<style>
  div {
    margin-top: var(--cds-spacing-05);
  }
</style>
