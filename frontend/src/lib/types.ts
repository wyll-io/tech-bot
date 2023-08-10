import { z } from 'zod';

export const form = {
	addTechnology: z.object({
		name: z.string().min(2),
		link: z.string().url('link must be an URL'),
		tags: z
			.string()
			.regex(
				/^((\w|\/|-|_)+|((\w|\/|-|_)+,)+\w+)$/gm,
				'tags must be a comma separated list of words'
			)
	}),
	searchTechnology: z.object({
		regex: z.string(),
		options: z.string().max(7),
		tags: z
			.optional(
				z
					.string()
					.regex(
						/^(?=\s*$)|^((\w|\/|-|_)+|((\w|\/|-|_)+,)+\w+)$/gm,
						'tags must be a comma separated list of words'
					)
			)
			.default('')
	})
};

export type Technology = {
	id: string;
	name: string;
	link: string;
	tags: string[];
	userId: string;
	createdAt: string;
	updatedAt: string;
};

export type OAuth2Response = {
	access_token: string;
	refresh_token: string;
	expires_in: number;
	token_type: string;
	scope: string;
};

type clientData = {
	clientID: string;
	clientSecret: string;
};

export type RequestToken = {
	grantType: 'authorization_code';
	code: string;
	redirectUri: string;
} & clientData;

export type RefreshToken = {
	grantType: 'refresh_token';
	refresh_token: string;
} & clientData;
