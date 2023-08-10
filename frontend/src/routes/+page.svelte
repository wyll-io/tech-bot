<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import Text from '$lib/components/inputs/text.svelte';
	import { toast } from '$lib/toast';
	import type { Technology } from '$lib/types';
	import { getContextClient, gql, mutationStore, queryStore } from '@urql/svelte';
	import type { ChangeEventHandler } from 'svelte/elements';
	import { writable } from 'svelte/store';
	import { superForm } from 'sveltekit-superforms/client';
	import type { PageData } from './$types';

	export let data: PageData;

	const client = getContextClient();
	const ltsTechnologies = writable<Technology[]>([]);
	const searchedTechnologies = writable<Technology[]>([]);
	const tbd = writable<string[]>([]);

	queryStore({
		client: client,
		query: gql`
			query {
				technologies {
					id
					name
					link
					tags
					userId
					createdAt
					updatedAt
				}
			}
		`
	}).subscribe(({ error, data }) => {
		if (error) {
			toast({ message: error.message, type: 'error' });
			console.log(error.message);
		} else if (data) {
			ltsTechnologies.set(data.technologies);
		}
	});

	const clearAddForm = (e: Event) => {
		e.preventDefault();

		$addForm.name = '';
		$addForm.link = '';
		$addForm.tags = '';
	};

	const {
		enhance: addEnhance,
		form: addForm,
		errors: addErrors
	} = superForm(data.add, {
		onError({ result }) {
			toast({
				message: result.error.message,
				type: 'error'
			});
		},
		onUpdated({ form }) {
			if (form.valid) {
				mutationStore({
					client: client,
					query: gql`
						mutation ($name: String!, $link: String!, $tags: [String!]!, $userId: String!) {
							createTechnology(name: $name, link: $link, tags: $tags, userId: $userId) {
								id
								name
								link
								tags
								userId
								createdAt
								updatedAt
							}
						}
					`,
					variables: {
						name: form.data.name,
						link: form.data.link,
						tags: form.data.tags.split(','),
						userId: data.id
					}
				}).subscribe(({ error, data }) => {
					if (error) {
						toast({ message: error.message, type: 'error' });
						console.log(error.message);
					} else if (data) {
						ltsTechnologies.update((v) => [data.createTechnology, ...v]);
						$addForm.name = '';
						$addForm.link = '';
						$addForm.tags = '';
					}
				});
			}
		},
		resetForm: true
	});

	const {
		enhance: searchEnhance,
		form: searchForm,
		errors: searchErrors
	} = superForm(data.search, {
		onError({ result }) {
			toast({
				message: result.error.message,
				type: 'error'
			});
		},
		onUpdated({ form }) {
			if (form.valid) {
				queryStore({
					client: client,
					query: gql`
						query ($name: String!, $options: String!, $tags: [String!]) {
							technology(name: $name, options: $options, tags: $tags) {
								id
								name
								link
								tags
								userId
								createdAt
								updatedAt
							}
						}
					`,
					variables: {
						name: form.data.regex,
						options: form.data.options,
						tags:
							form.data.tags.includes(',') || form.data.tags.length > 0
								? form.data.tags.split(',')
								: null
					}
				}).subscribe(({ error, data }) => {
					if (error) {
						toast({ message: error.message, type: 'error' });
						console.log(error.message);
					} else if (data) {
						searchedTechnologies.set(data.technology);
					}
				});
			}
		}
	});

	const updateTBD = (id: string | 'all'): ChangeEventHandler<HTMLInputElement> => {
		return (el) => {
			if (el.target && (el.target as Record<string, any>).checked) {
				if (id === 'all') tbd.set($searchedTechnologies.map((tech) => tech.id));
				else tbd.update((v) => [...v, id]);
				return;
			}

			tbd.update((v) => {
				const i = v.indexOf(id);
				if (i > -1) v.splice(i, 1);
				return v;
			});
		};
	};
	const deleteTBD = () => {
		mutationStore({
			client,
			query: gql`
				mutation ($ids: [Uuid!]!) {
					deleteTechnologies(ids: $ids)
				}
			`,
			variables: {
				ids: $tbd
			}
		}).subscribe(({ error }) => {
			if (error) {
				toast({ message: error.message, type: 'error' });
			} else {
				toast({ message: 'technologies deleted!', type: 'success' });
				invalidateAll().catch((error) => toast({ message: error.message, type: 'error' }));
			}
		});
	};
	const formatDate = (date: Date) => {
		return `${date.getDate()}/${date.getMonth()}/${date.getFullYear()} ${date.getHours()}:${date.getMinutes()}`;
	};
</script>

<div class="flex flex-col items-center">
	<h1 class="text-xl font-bold text-center">Latest added technologies</h1>
	<div class="card w-11/12 bg-base-200 shadow-inner h-auto my-4">
		<div class="card-body items-center text-center">
			<div class="overflow-x-auto">
				<table class="table">
					<thead>
						<tr>
							<th />
							<th>ID</th>
							<th>Name</th>
							<th>Link</th>
							<th>Tags</th>
							<th>Created at</th>
							<th>Updated at</th>
							<th>User</th>
						</tr>
					</thead>
					<tbody>
						{#each $ltsTechnologies as tech, index}
							<tr>
								<th>{index + 1}</th>
								<td>{tech.id}</td>
								<td>{tech.name}</td>
								<td>{tech.link}</td>
								<td>{tech.tags}</td>
								<td>{formatDate(new Date(Number(tech.createdAt) * 1000))}</td>
								<td
									>{tech.updatedAt
										? formatDate(new Date(Number(tech.updatedAt) * 1000))
										: tech.updatedAt}</td
								>
								<td>{tech.userId}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>
	</div>

	<h1 class="text-xl font-bold text-center">Add a technology</h1>
	<div class="card w-11/12 bg-base-200 shadow-inner h-auto my-4">
		<div class="card-body items-center text-center">
			<form method="POST" action="?/add" use:addEnhance class="flex flex-col space-y-6">
				<div class="flex flex-row space-x-6">
					<Text
						bind:value={$addForm.name}
						errors={addErrors}
						label="Name"
						name="name"
						placeholder="GitHub"
					/>

					<Text
						bind:value={$addForm.link}
						errors={addErrors}
						label="Link"
						name="link"
						placeholder="https://github.com"
					/>

					<Text
						bind:value={$addForm.tags}
						errors={addErrors}
						label="Tags"
						name="tags"
						placeholder="CI/CD,git"
					/>
				</div>

				<div class="flex flex-row space-x-4 self-center">
					<button class="btn btn-success w-fit">Add</button>
					<button class="btn btn-error w-fit" on:click={clearAddForm}>Clear form</button>
				</div>
			</form>
		</div>
	</div>

	<h1 class="text-xl font-bold text-center">Delete a technology</h1>
	<div class="card w-11/12 bg-base-200 shadow-inner h-auto my-4">
		<div class="card-body items-center text-center">
			<form method="POST" action="?/search" use:searchEnhance class="flex flex-col space-y-6">
				<div class="join self-center">
					<Text
						bind:value={$searchForm.regex}
						errors={searchErrors}
						label="Regular expression"
						name="regex"
						placeholder="github|etree"
						inputClasses="join-item"
						showtopLabel={false}
					/>

					<Text
						bind:value={$searchForm.options}
						errors={searchErrors}
						label="Options"
						name="options"
						placeholder="valid: imsRUux"
						inputClasses="join-item"
						showtopLabel={false}
					/>
					<Text
						bind:value={$searchForm.tags}
						errors={searchErrors}
						label="Tags"
						name="tags"
						placeholder="git,rust,library"
						inputClasses="join-item"
						showtopLabel={false}
					/>

					<button class="btn btn-success join-item">Search</button>
				</div>

				{#if $searchedTechnologies.length > 0}
					<div class="overflow-x-auto">
						<table class="table">
							<thead>
								<tr>
									<th>
										<label>
											<input type="checkbox" class="checkbox" on:change={updateTBD('all')} />
										</label>
									</th>
									<th>Name</th>
									<th>Link</th>
									<th>Tags</th>
									<th>Created at</th>
									<th>Updated at</th>
									<th>User</th>
								</tr>
							</thead>
							<tbody>
								{#each $searchedTechnologies as tech}
									<tr>
										<th>
											<label>
												<input type="checkbox" class="checkbox" on:change={updateTBD(tech.id)} />
											</label>
										</th>
										<td>{tech.name}</td>
										<td>{tech.link}</td>
										<td>{tech.tags}</td>
										<td>{formatDate(new Date(Number(tech.createdAt) * 1000))}</td>
										<td
											>{tech.updatedAt
												? formatDate(new Date(Number(tech.updatedAt) * 1000))
												: tech.updatedAt}</td
										>
										<td>{tech.userId}</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
					<div class="flex flex-row self-center">
						<button class="btn btn-error w-fit" on:click={deleteTBD}>Delete</button>
					</div>
				{/if}
			</form>
		</div>
	</div>
</div>
