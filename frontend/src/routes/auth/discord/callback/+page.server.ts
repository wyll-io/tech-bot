import { env } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';
import type { OAuth2Response } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url, cookies }) => {
	const code = url.searchParams.get('code');
	if (!code) {
		console.error(`failed to get code in callback url: ${url}`);
		return { ok: false, uri: '/login', status: 302 };
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
			redirect_uri: `${publicEnv.PUBLIC_ORIGIN}/auth/discord/callback`,
			code
		})
	});

	if (!rsp.ok) return { ok: false, uri: '/login', status: 302 };

	const { access_token, refresh_token, expires_in }: OAuth2Response = await rsp.json();

	cookies.set('access-token', access_token, {
		maxAge: expires_in,
		httpOnly: true,
		sameSite: true,
		path: '/',
		secure: process.env.NODE_ENV === 'production'
	});
	cookies.set('refresh-token', refresh_token, {
		maxAge: expires_in * 2,
		httpOnly: true,
		sameSite: true,
		path: '/',
		secure: process.env.NODE_ENV === 'production'
	});

	return { ok: true, uri: '/', status: 302 };
};
