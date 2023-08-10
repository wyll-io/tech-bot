import { toast } from '$lib/toast';
import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals: { session }, cookies }) => {
	if (session) {
		const rsp = await fetch('https://discord.com/api/v10/users/@me', {
			headers: {
				Authorization: `Bearer ${cookies.get('access-token')}`
			}
		});

		if (!rsp.ok) {
			console.log("failed to fetch user's information");
			toast({ type: 'error', message: "failed to fetch user's information" });
			return {};
		}

		const { id, username, avatar } = await rsp.json();

		return {
			id,
			username,
			avatar: `https://cdn.discordapp.com/avatars/${id}/${avatar}.png`
		};
	}

	return {};
};
