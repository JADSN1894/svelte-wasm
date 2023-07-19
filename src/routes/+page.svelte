<script lang="ts">
	import { onMount } from 'svelte';
	import init, { add, dispatch, bip_0039_mnemonic_phrase } from '$lib/encoder/encoder';

	let input = '';
	let isLoading = true;
	let words: string[] = [];
	let output = '';

	onMount(async () => {
		await init().then((_) => {
			isLoading = false;
			words = bip_0039_mnemonic_phrase().split(' ');
			output = dispatch(JSON.stringify({ cmd: 'md5', data: input }));
		});
	});

	let selected = 'md5';

	function onChangeEncodeType(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		selected = event.currentTarget.value;
		output = dispatch(JSON.stringify({ cmd: event.currentTarget.value, data: input }));
	}
</script>

{#if isLoading}
	<h1>LOADING</h1>
{:else}
	<div class="flex flex-col items-center justify-center h-screen overflow-hidden gap-y-4">
		<div class="grid grid-cols-4 gap-4 border-dashed border-2 border-primary p-4">
			{#each words as word, i (i)}
				<span class="text-primary font-bold">{word}</span>
			{/each}
		</div>
		<div class="flex items-center justify-center gap-x-2">
			<label class="flex items-center justify-center">
				<input
					class="radio radio-primary"
					checked={selected === 'md5'}
					on:change={onChangeEncodeType}
					type="radio"
					name="amount"
					value="md5"
				/> <span class="pl-2">md5</span>
			</label>
			<label class="flex items-center justify-center">
				<input
					class="radio radio-primary"
					checked={selected === 'sha1'}
					on:change={onChangeEncodeType}
					type="radio"
					name="amount"
					value="sha1"
				/> <span class="pl-2">sha1</span>
			</label>
			<label class="flex items-center justify-center">
				<input
					class="radio radio-primary"
					checked={selected === 'sha256'}
					on:change={onChangeEncodeType}
					type="radio"
					name="amount"
					value="sha256"
				/> <span class="pl-2">sha256</span>
			</label>
			<label class="flex items-center justify-center">
				<input
					class="radio radio-primary"
					checked={selected === 'sha512'}
					on:change={onChangeEncodeType}
					type="radio"
					name="amount"
					value="sha512"
				/> <span class="pl-2">sha512</span>
			</label>
		</div>
		<input
			class="input input-bordered input-primary w-full max-w-xs items-center"
			type="text"
			bind:value={input}
			on:input={() => {
				output = dispatch(JSON.stringify({ cmd: selected, data: input }));
			}}
		/>

		<p class="text-md items-start">{output}</p>
	</div>
{/if}
