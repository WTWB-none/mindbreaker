<script>
	import Settings from '../Settings.svelte';
	import { invoke } from '@tauri-apps/api/core';
	let show_component = $state();
	let settings_list = $state('');
	async function openSettings() {
		settings_list = await invoke('get_settings_list');
		show_component = true;
		console.log(settings_list);
	}
</script>

<div
	class="settings-container"
	tabindex="-100000"
	onkeydown={() => console.log('keydown')}
	onclick={openSettings}
	aria-label="Настройки"
	role="button"
>
	<img src="settings.png" alt="Settings" class="settings-icon" />
	<h1>Настройки</h1>
</div>
{#if show_component}
	<Settings bind:show={show_component} bind:settings={settings_list} />
{/if}

<style>
	.settings-container {
		max-width: 100%;
		display: flex;
		justify-content: flex-start;
		color: #ababab;
		font-size: 0.7em;
		align-items: center;
		gap: 1em;
		padding: 0.7em;
		cursor: pointer;
	}

	.settings-icon {
		width: 40px;
	}

	.settings-container h1 {
		margin: 0;
		transition: ease-in 0.2s;
		font-weight: 200;
	}
</style>
