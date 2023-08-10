<script lang="ts">
	import type { Writable } from 'svelte/store';
	import type { ValidationErrors } from 'sveltekit-superforms';
	import type { ZodObject } from 'zod';

	export let value: string;
	export let errors: Writable<ValidationErrors<ZodObject<Record<string, any>>>>;
	export let name: string;
	export let label: string;
	export let options: {
		selected: boolean;
		disabled: boolean;
		value: string;
		text: string;
	}[];
</script>

<div
	class="form-control w-full max-w-xs {$errors[name] ? 'input-error' : ''}"
	aria-invalid={$errors[name] ? 'true' : undefined}
>
	<label for={name} class="label">
		<span class="label-text">{label}</span>
	</label>
	<select class="select select-bordered" bind:value {name}>
		{#each options as opt}
			<option value={opt.value} selected={opt.selected} disabled={opt.disabled}>{opt.text}</option>
		{/each}
		<option disabled selected>Pick one</option>
		<option>Star Wars</option>
		<option>Harry Potter</option>
		<option>Lord of the Rings</option>
		<option>Planet of the Apes</option>
		<option>Star Trek</option>
	</select>
	{#if $errors[name]}
		<label for={name} class="label">
			<span class="label-text-alt text-red-600">{$errors[name]}</span>
		</label>
	{/if}
</div>
