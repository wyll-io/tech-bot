import { env } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';
import { redirect, type RequestEvent } from '@sveltejs/kit';
import type { OAuth2Response } from './types';

export default async (event: RequestEvent) => {
	const refreshToken = event.cookies.get('refresh-token');
	if (!refreshToken) throw redirect(302, '/login');

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

	if (!rsp.ok) throw redirect(302, '/login');

	const { access_token, expires_in }: OAuth2Response = await rsp.json();
	event.cookies.set('access-token', access_token, {
		domain: publicEnv.PUBLIC_ORIGIN,
		maxAge: expires_in,
		expires: new Date(Date.now() + expires_in),
		httpOnly: true,
		sameSite: true,
		path: '/'
	});
};
