<script lang="ts">
	import { afterNavigate } from '$app/navigation';
	import { base } from '$app/paths';
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';

	let previousPage: string = base;

	afterNavigate(({ from }) => {
		previousPage = from?.url.pathname || previousPage;
	});
</script>

<section class="bg-white dark:bg-gray-900">
	<div class="container flex items-center h-full px-6 py-12 mx-auto">
		<div class="flex flex-col items-center max-w-sm mx-auto text-center">
			<p class="p-3 text-sm font-medium text-blue-500 rounded-full bg-blue-50 dark:bg-gray-800">
				<Icon icon="material-symbols:error-outline" width="32" />
			</p>

			<h1 class="mt-3 text-2xl font-semibold text-gray-800 dark:text-white md:text-3xl">
				{$page.status}
			</h1>
			<h1 class="mt-3 text-2xl font-semibold text-gray-800 dark:text-white md:text-3xl">
				{$page.error?.message}
			</h1>

			<div class="flex items-center w-full mt-6 gap-x-3 shrink-0 sm:w-auto">
				<a
					href={previousPage}
					rel="prefetch"
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-gray-700 transition-colors duration-200 bg-white border rounded-lg gap-x-2 sm:w-auto dark:hover:bg-gray-800 dark:bg-gray-900 hover:bg-gray-100 dark:text-gray-200 dark:border-gray-700"
				>
					<Icon icon="radix-icons:pin-left" />
					Go back
				</a>

				<a
					href="/"
					class="flex items-center justify-center w-1/2 px-5 py-2 text-sm text-white transition-colors duration-200 bg-blue-500 rounded-lg sm:w-auto hover:bg-blue-600 dark:hover:bg-blue-500 dark:bg-blue-600"
				>
					<Icon icon="clarity:home-line" class="mr-2" />
					Take me home
				</a>
			</div>
		</div>
	</div>
</section>
