// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {
		// 	status: number;
		// 	message: string;
		// }

		interface Locals {
			session: boolean;
		}
		// interface PageData {}
		// interface Platform {}
	}
}

export {};
