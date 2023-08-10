import { form as formType } from '$lib/types';
import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms/server';
import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	return {
		add: await superValidate(formType.addTechnology),
		search: await superValidate(formType.searchTechnology)
	};
};

export const actions: Actions = {
	add: async ({ request }) => {
		const form = await superValidate(request, formType.addTechnology);

		if (!form.valid) return fail(400, { form });

		return { form };
	},
	search: async ({ request }) => {
		const form = await superValidate(request, formType.searchTechnology);

		if (!form.valid) return fail(400, { form });

		return { form };
	}
};
