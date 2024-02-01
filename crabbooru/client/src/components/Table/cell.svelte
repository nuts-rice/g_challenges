<svelte:options tag = "cobalt-cell"/>
<script>
    import {ImageLoader, InlineLoading} from "carbon-components-svelte";
    export let placeholder = "false";
    const parsePlaceholder = (/** @type {string} */ placeholder) => { try{ return JSON.parse(placeholder); } catch(e) { return false; } };
    const imgHolder = parsePlaceholder(placeholder); 
    </script>

    <div class="cell">
        {#if imgHolder}
            <div>
            <ImageLoader src="https://testbooru-cdn.donmai.us/sample/c9/60/sample-c960357a64ef44742509f9b1983121ab.jpg" alt="placeholder" >
            <svelte:fragment slot="loading">
            <InlineLoading />
            </svelte:fragment>
            <svelte:fragment slot="error"> An error occured </svelte:fragment>
            </ImageLoader>
            </div>
        {:else}
        {/if}
    </div>
    <style>
        .cell {
            display: flex;
            flex-direction: column;
            text-align: center;            
            align-items: center;
            justify-content: center;
            border: 1px solid black;
            padding: 10px;
            margin: 10px;
        }
        img {
            width: 100%;
            padding: 5px;
            height: auto;

        }
        .description {
            font-family: Arial, Helvetica, sans-serif;
            color:black;
            background-color: #6666ff;
            padding: 2px 0;
            font-size: 1.5em;
            text-align: center;
        }
    </style>