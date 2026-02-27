// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import type { ZodiosInstance } from '@zodios/core';
import type { Api } from '@crown-shy/api-client/api';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			api: ZodiosInstance<Api>;
		}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
