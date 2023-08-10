<script lang="ts">
	import type { Writable } from 'svelte/store';
	import type { ValidationErrors } from 'sveltekit-superforms';
	import type { ZodObject } from 'zod';

	export let value: string;
	export let errors: Writable<ValidationErrors<ZodObject<Record<string, any>>>>;
	export let name: string;
	export let label: string;
	export let placeholder: string;
	export let inputClasses: string = '';
	export let showtopLabel = true;
	export let disabled = false;
</script>

<div class="flex flex-col items-start">
	{#if showtopLabel}
		<label for={name} class="label">
			<span class="label-text">{label}</span>
		</label>
	{/if}
	<input
		type="text"
		{name}
		class="input input-bordered max-w-xs w-full {inputClasses} {$errors[name] ? 'input-error' : ''}"
		{placeholder}
		aria-invalid={$errors[name] ? 'true' : undefined}
		bind:value
		{disabled}
	/>
	{#if $errors[name]}
		<label for={name} class="label">
			<span class="label-text-alt text-red-600">{$errors[name]}</span>
		</label>
	{/if}
</div>
