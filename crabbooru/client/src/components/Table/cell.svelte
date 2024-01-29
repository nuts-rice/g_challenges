<svelte:options tag = "cobalt-cell"/>
<script>
    export let placeholder = "false";
    let imgHolder = JSON.parse(placeholder);
    </script>

    <div class="cell">
        {#if imgHolder}
            <div>
            <img src="https://testbooru.donmai.us/posts?tags=touhou+&z=5" alt="placeholder" />
            <div class = "description"> This is a placeholder image </div>
            </div>
            {:else}
            <slot />
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