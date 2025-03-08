<script>
	import DashboardNameForm from './DashboardNameForm.svelte';
	export const name = $state('');
	const plugins = $state([
		{ id: 'plugin1', name: 'clock' },
		{ id: 'plugin2', name: 'quote' }
	]);
	async function loadDashboardPlugin(plugname) {
		try {
			const module = await import(`./plugins/${plugname}.svelte`);
			return module.default;
		} catch (error) {
			console.error(`Failed to load plugin ${plugname}:`, error);
			return null;
		}
	}
</script>

{#if name === ''}
	<DashboardNameForm />
{:else}
	<div class="main-dashboard-container">
		<h1>Добро пожаловать, {name}</h1>
		<div class="plugins-container">
			{#each plugins as plugin}
				{#await loadDashboardPlugin(plugin.name) then Component}
					{#if Component}
						<div class="card">
							<Component />
						</div>
					{/if}
				{/await}
			{/each}
		</div>
	</div>
{/if}

<style>
	.plugins-container {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin: 5%;
		max-width: 90%;
		flex-wrap: wrap;
	}
	.card {
		padding: 50px;
		min-width: 10%;
		border-radius: 15%;
		border: 1px solid #ccc;
	}
	.card:hover {
		box-shadow: 10px 10px 50px gray;
	}
</style>
