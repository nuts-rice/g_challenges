<script lang="ts">
  import SourcesWindow from "./SourcesWindow.svelte";
  import MainWindow from "./MainWindow.svelte";
  import Table from "../components/Table/table.svelte";
  import SearchBar from "../components/Search/searchBar.svelte";
  import SearchTab from "../components/Tabs/searchTab.svelte";
  import FavoritesTab from "../components/Tabs/favoritesTab.svelte";
  import {Tabs, Tab, TabContent} from "carbon-components-svelte";
</script>

<head>
  <title> CrabbooruUi </title>
  <meta content="width=device-width, initial-scale=1" name="viewport" />
</head>
<app-root></app-root>
<div>Tags List</div>
<section>
  <Tabs>
    <Tab label="Search">
      <svelte:fragment slot="content">
      <TabContent> 
        <SearchTab />
        <SearchBar/>
      </TabContent>
      </svelte:fragment>
    </Tab>
    <Tab label="Favorites">
      <svelte:fragment slot="content">
      <TabContent> 
        <FavoritesTab />
      </TabContent>
      </svelte:fragment>
    </Tab>    
  </Tabs> 
</section>   
<section>
  <SearchBar />
</section>
<section>
  Image gallery goes here

  <Table />
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    flex: 0.6;
  }

  h1 {
    width: 100%;
  }

  .welcome {
    display: block;
    position: relative;
    width: 100%;
    height: 0;
    padding: 0 0 calc(100% * 495 / 2048) 0;
  }

  .welcome img {
    position: absolute;
    width: 100%;
    height: 100%;
    top: 0;
    display: block;
  }
  .sources {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
  }
  .tab-content {
    overflow: scroll; 
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
  }
</style>
