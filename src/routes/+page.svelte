<script lang="ts">
	import { onMount } from 'svelte'

	let count = 0
	onMount(() => {
		const interval = setInterval(() => count++, 1000)
		return () => clearInterval(interval)
	})
</script>

<div class="main">
	<h1 class="title has-text-primary">Svelte Template</h1>
	<p>Edit <code>src/routes/+page.svelte</code> to trigger HMR</p>
	<p>Counter: <code>{count}</code></p>
	<img src="/tauri-svelte-template.png" alt="Tauri Svelte Template" height="200">
</div>
<style lang="scss">
 .main {
    margin: 10px 0;
    font-family: Arial, Helvetica, sans-serif;
    font-size: 18px;
    background-color: #111318;
    color: #f2f2f2;
    text-align: center;
	  width: 100%;
  }

  code {
    background: #282b33;
    padding: 2px 5px;
    border-radius: 4px;
  }
</style>
