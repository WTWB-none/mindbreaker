<script>
	import { slide } from 'svelte/transition';
	let fname = $state('');
	function getname() {
		if (fname === '') {
			alert('Как скажете, господин)');
			fname = 'Господин';
		}
		console.log(fname);
	}
</script>

<div in:slide={{ duration: 300 }}>
	<div class="background-blur">
		<div class="form-container">
			<form>
				<h1>Как мне тебя называть?</h1>
				<input type="text" id="input-login" bind:value={fname} />
				<input type="submit" value="Зови меня господин)" onclick={getname} />
			</form>
		</div>
	</div>
</div>

<style>
	.form-container {
		position: absolute;
		left: 0px;
		top: 0px;
		width: 100%;
		min-height: 100vh;
		margin-top: 0px;
		background: radial-gradient(rgba(0, 0, 0, 1), rgba(255, 255, 255, 0.1));
		color: #ababab;
		display: flex;
		align-items: center;
		justify-content: center;
	}
	form {
		border: 3px solid #ababab;
		border-radius: 15px;
		padding: 10%;
		max-height: 50vh;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		&:hover {
			border-color: white;
		}
	}

	input[type='text'] {
		outline: 1px solid white;
		border: none;
		color: #ababab;
		border-radius: 15px;
		font-size: 2em;
		padding: 3vh;
		margin: 5px;
		width: 100%;
		background: #0b0a0c;
	}

	input[type='submit'] {
		border-radius: 15px;
		margin-top: 5vh;
		outline: none;
		border: none;
		background-color: #ababab;
		width: 50%;
		font-size: 1em;
		padding: 2vh;
		cursor: pointer;
	}
</style>
