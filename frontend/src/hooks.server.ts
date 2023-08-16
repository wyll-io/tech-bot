import refresh from '$lib/refreshToken';
import { redirect, type Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

const protectedRoutes = ['/'];

const handleAuth: Handle = async ({ resolve, event }) => {
	const refreshToken = event.cookies.get('refresh-token');
	const accessToken = event.cookies.get('access-token');

	if (refreshToken && !accessToken) await refresh(event);

	return await resolve(event);
};

const handleUserSession: Handle = async ({ resolve, event }) => {
	if (event.cookies.get('access-token') && event.cookies.get('refresh-token')) {
		const rsp = await event.fetch('https://discord.com/api/v10/users/@me', {
			headers: {
				Authorization: `Bearer ${event.cookies.get('access-token')}`
			}
		});

		if (!rsp.ok) console.error(`failed to get user session: ${rsp.status} ${await rsp.text()}`);

		const { id, username, avatar } = await rsp.json();

		event.locals.user = {
			id,
			username,
			avatar: `https://cdn.discordapp.com/avatars/${id}/${avatar}.png`
		};
	}

	return await resolve(event);
};

const handleGuard: Handle = async ({ resolve, event }) => {
	if (!event.locals.user && protectedRoutes.includes(event.url.pathname))
		throw redirect(302, '/login');
	else if (
		(event.locals.user && event.url.pathname === '/login') ||
		(event.locals.user && event.url.pathname.includes('/auth'))
	)
		throw redirect(302, '/');

	return await resolve(event);
};

export const handle: Handle = sequence(handleAuth, handleUserSession, handleGuard);
