import { redirect, type Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

const protectedRoutes = ['/'];

const auth: Handle = async ({ resolve, event }) => {
	console.log(`handle auth for route: ${event.url.pathname}`);

	const refreshToken = event.cookies.get('refresh-token');
	let accessToken = event.cookies.get('access-token');

	console.log(`refresh token: ${refreshToken}`);
	console.log(`access token: ${accessToken}`);

	if (!accessToken && refreshToken) {
		const rsp = await event.fetch('/auth/discord', {
			method: 'PUT',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({ refreshToken })
		});

		if (!rsp.ok) throw redirect(302, '/login');

		accessToken = event.cookies.get('access-token');
	}

	// * grab the access token again, in case it was just refreshed
	event.locals.session = !!(event.cookies.get('access-token') && refreshToken);

	return await resolve(event);
};

const guard: Handle = async ({ resolve, event }) => {
	if (protectedRoutes.includes(event.url.pathname) && !event.locals.session) {
		console.warn(`authentication failed for: ${event.url.pathname}`);
		throw redirect(302, '/login');
	} else if (
		(event.url.pathname === '/login' || event.url.pathname.includes('/auth')) &&
		event.locals.session
	) {
		console.log('already authenticated. redirecting to home page');
		throw redirect(302, '/');
	}

	return await resolve(event);
};

export const handle: Handle = sequence(auth, guard);
