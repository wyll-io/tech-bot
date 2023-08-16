<script lang="ts">
	import { env } from '$env/dynamic/public';
	import Toast from '$lib/components/toast.svelte';
	import Icon from '@iconify/svelte';
	import { Client, cacheExchange, fetchExchange, setContextClient } from '@urql/svelte';
	import '../app.css';
	import type { PageData } from './$types';

	export let data: PageData;

	$: ({ user } = data);

	setContextClient(
		new Client({
			url: env.PUBLIC_GRAPHQL_ENDPOINT,
			exchanges: [cacheExchange, fetchExchange]
		})
	);
</script>

<Toast />

<div class="flex flex-col min-h-screen justify-center items-center">
	<div class="navbar bg-base-content text-white">
		<div class="flex-1">
			<a class="btn btn-ghost normal-case text-xl" href="/">tech-bot</a>
		</div>

		<div class="flex-1 justify-end space-x-5 mr-5">
			{#if user}
				<div class="tooltip tooltip-bottom tooltip-info" data-tip="Copy ID">
					<button
						class="btn btn-ghost p-0 normal-case"
						on:click={() => navigator.clipboard.writeText(user.id)}
					>
						{user.username}
					</button>
				</div>
				<div class="tooltip tooltip-bottom tooltip-info" data-tip="Logout">
					<a href="/logout">
						<Icon icon="material-symbols:logout" width="28" />
					</a>
				</div>
				<div class="avatar">
					<div class="w-8 rounded-full">
						<img src={user.avatar} alt="user's avatar" />
					</div>
				</div>
			{/if}
		</div>
	</div>

	<div class="flex-grow flex items-center">
		<slot />
	</div>

	<footer class="footer footer-center p-4 bg-base-300 text-base-content">
		<div>
			<p>Copyright © 2023 - Made by Antoine Langlois</p>
		</div>
	</footer>
</div>
