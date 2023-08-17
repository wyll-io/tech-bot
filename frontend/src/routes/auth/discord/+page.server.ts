import { env } from '$env/dynamic/private';
import { env as publicEnv } from '$env/dynamic/public';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const params = new URLSearchParams({
		client_id: env.CLIENT_ID,
		redirect_uri: `${publicEnv.PUBLIC_ORIGIN}/auth/discord/callback`,
		response_type: 'code',
		scope: 'identify'
	});

	throw redirect(302, `https://discord.com/api/oauth2/authorize?${params.toString()}`);
};
