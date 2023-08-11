// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}

		interface Locals {
			session: boolean;
		}
		interface PageData {
			user?: {
				id: string;
				username: string;
				avatar: string;
			};
		}
		// interface Platform {}
	}
}

export {};
