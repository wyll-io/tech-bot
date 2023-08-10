import { env } from '$env/dynamic/private';
import type { OAuth2Response } from '$lib/types';
import { error, redirect, type Handle } from '@sveltejs/kit';
import { sequence } from '@sveltejs/kit/hooks';

const protectedRoutes = ['/'];

const auth: Handle = async ({ resolve, event }) => {
	const refreshToken = event.cookies.get('refresh-token');
	const accessToken = event.cookies.get('access-token');

	if (!accessToken && refreshToken) {
		const rsp = await fetch('https://discord.com/api/oauth2/token', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams({
				client_id: env.CLIENT_ID,
				client_secret: env.CLIENT_SECRET,
				grant_type: 'refresh_token',
				refresh_token: refreshToken
			})
		});

		if (!rsp.ok) {
			console.error(`failed to refresh token: ${rsp.status}`);
			event.cookies.delete('refresh-token');
			throw redirect(303, '/login');
		}

		const { access_token, expires_in }: OAuth2Response = await rsp.json();
		event.cookies.set('access-token', access_token, {
			maxAge: expires_in,
			expires: new Date(Date.now() + expires_in),
			httpOnly: true,
			sameSite: true,
			path: '/'
		});
	}

	// * grab the access token again, in case it was just refreshed
	event.locals.session = !!(event.cookies.get('access-token') && refreshToken);

	return await resolve(event);
};

const handleAuth: Handle = async ({ resolve, event }) => {
	if (event.locals.session && event.url.pathname.includes('/auth')) throw redirect(303, '/');
	else if (event.locals.session && event.url.pathname === '/logout') {
		event.cookies.delete('access-token');
		event.cookies.delete('refresh-token');

		throw redirect(303, '/login');
	} else if (event.locals.session) return await resolve(event);

	if (event.url.origin !== env.ORIGIN) {
		console.error(`invalid origin. ${event.url.origin}`);
		throw error(403, 'invalid origin');
	}

	if (event.url.pathname === '/auth/discord') {
		const params = new URLSearchParams({
			client_id: env.CLIENT_ID,
			redirect_uri: `${env.ORIGIN}/auth/callback/discord`,
			response_type: 'code',
			scope: 'identify'
		});
		throw redirect(302, `https://discord.com/api/oauth2/authorize?${params.toString()}`);
	} else if (event.url.pathname === '/auth/callback/discord') {
		const code = event.url.searchParams.get('code');
		if (!code) {
			console.error(`failed to get code in callback url: ${event.url}`);
			throw redirect(303, '/login');
		}

		const rsp = await fetch('https://discord.com/api/oauth2/token', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: new URLSearchParams({
				client_id: env.CLIENT_ID,
				client_secret: env.CLIENT_SECRET,
				grant_type: 'authorization_code',
				redirect_uri: `${env.ORIGIN}/auth/callback/discord`,
				code
			})
		});

		if (!rsp.ok) throw redirect(303, '/login');

		const { access_token, refresh_token, expires_in }: OAuth2Response = await rsp.json();
		event.cookies.set('access-token', access_token, {
			maxAge: expires_in,
			expires: new Date(Date.now() + expires_in),
			httpOnly: true,
			sameSite: true,
			path: '/'
		});
		event.cookies.set('refresh-token', refresh_token, {
			maxAge: Date.now() + 60 * 60 * 24 * 30,
			expires: new Date(Date.now() + 60 * 60 * 24 * 30),
			httpOnly: true,
			sameSite: true,
			path: '/'
		});

		console.info('successfully authenticated user');
		throw redirect(303, '/');
	}

	return await resolve(event);
};

const guard: Handle = async ({ resolve, event }) => {
	if (protectedRoutes.includes(event.url.pathname) && !event.locals.session) {
		console.warn(`authentication failed for: ${event.url.pathname}`);
		throw redirect(303, '/login');
	} else if (event.url.pathname === '/login' && event.locals.session) throw redirect(303, '/');

	return await resolve(event);
};

export const handle: Handle = sequence(auth, guard, handleAuth);
