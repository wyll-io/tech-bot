import { env } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';
import type { OAuth2Response } from '$lib/types';
import { serialize } from 'cookie';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = () => {
	const params = new URLSearchParams({
		client_id: env.CLIENT_ID,
		redirect_uri: `${publicEnv.PUBLIC_ORIGIN}/auth/callback/discord`,
		response_type: 'code',
		scope: 'identify'
	});

	return new Response(null, {
		headers: {
			Location: `https://discord.com/api/oauth2/authorize?${params.toString()}`
		},
		status: 302
	});
};

export const PUT: RequestHandler = async ({ request, fetch }) => {
	const { refreshToken } = await request.json();

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

	if (!rsp.ok) return new Response(null, { status: 401 });

	const { access_token, expires_in }: OAuth2Response = await rsp.json();

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

	return new Response(null, {
		status: 302,
		headers
	});
};
