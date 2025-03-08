<script>
	import WelcomeAnim from '$lib/components/WelcomeAnim.svelte';
	import Dashboard from '$lib/components/Dashboard.svelte';
	const AnimShow = false;
</script>

<div class="main view">
	{#if AnimShow}
		<WelcomeAnim />
	{/if}
	{#if !AnimShow}
		<Dashboard />
	{/if}
</div>

<style>
	.view {
		background-color: #16181f;
		width: 100%;
		margin: 0;
		padding: 0;
	}
</style>
