<script>
  import SourcesWindow from './SourcesWindow.svelte';
  import MainWindow from './MainWindow.svelte';
  import Table from '../components/Table/table.svelte';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Booru app" />
</svelte:head>
<section>
	<SourcesWindow />

  
</section>


<section>
//<MainWindow />
</section>

<section>
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
</style>
