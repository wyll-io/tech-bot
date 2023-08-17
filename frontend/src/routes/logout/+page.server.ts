import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies }) => {
	cookies.set('access-token', '', { maxAge: -1 });
	cookies.set('refresh-token', '', { maxAge: -1 });

	throw redirect(302, '/login');
};
