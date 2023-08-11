import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async (event) => {
	event.cookies.delete('access-token');
	event.cookies.delete('refresh-token');

	throw redirect(302, '/login');
};
