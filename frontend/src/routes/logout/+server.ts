import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ cookies }) => {
	cookies.set('access-token', '', { maxAge: -1 });
	cookies.set('refresh-token', '', { maxAge: -1 });

	throw redirect(302, '/login');
};
