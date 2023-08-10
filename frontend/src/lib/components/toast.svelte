<script lang="ts">
	import { deleteToast, toasts } from '$lib/toast';
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
</script>

{#if $toasts}
	<div class="toast toast-top toast-end z-50">
		{#each Object.entries($toasts) as [id, toast]}
			<div class="alert alert-{toast.type}" transition:fade>
				<span>{toast.message}</span>
				<button
					class="btn btn-ghost w-fit"
					on:click={() => {
						deleteToast(id);
					}}
				>
					<Icon icon="iwwa:delete" />
				</button>
			</div>
		{/each}
	</div>
{/if}
