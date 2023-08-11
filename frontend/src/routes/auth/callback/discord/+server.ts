import { env } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';
import type { OAuth2Response } from '$lib/types';
import { redirect } from '@sveltejs/kit';
import { serialize } from 'cookie';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async (event) => {
	const code = event.url.searchParams.get('code');
	if (!code) {
		console.error(`failed to get code in callback url: ${event.url}`);
		throw redirect(302, '/login');
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
			redirect_uri: `${publicEnv.PUBLIC_ORIGIN}/auth/callback/discord`,
			code
		})
	});

	if (!rsp.ok) throw redirect(302, '/login');

	const { access_token, refresh_token, expires_in }: OAuth2Response = await rsp.json();

	const headers = new Headers();
	headers.set('Location', '/');
	headers.append(
		'Set-Cookie',
		serialize('access-token', access_token, {
			domain: publicEnv.PUBLIC_ORIGIN,
			maxAge: expires_in,
			expires: new Date(Date.now() + expires_in),
			httpOnly: true,
			sameSite: true,
			path: '/'
		})
	);
	headers.append(
		'Set-Cookie',
		serialize('refresh-token', refresh_token, {
			domain: publicEnv.PUBLIC_ORIGIN,
			maxAge: 60 * 60 * 24 * 7,
			expires: new Date(Date.now() + 60 * 60 * 24 * 7),
			httpOnly: true,
			sameSite: true,
			path: '/'
		})
	);

	return new Response(null, {
		status: 302,
		headers
	});
};
