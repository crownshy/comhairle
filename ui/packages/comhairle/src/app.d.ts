// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
import type { createApiClient } from '@crownshy/api-client/client';
import type { UserActions } from '@crownshy/api-client/api';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			api: ReturnType<typeof createApiClient>;
		}
		interface PageData {
			systemActions?: UserActions | null;
			conversationActions?: UserActions | null;
			organizationActions?: UserActions | null;
			resourceActions?: (UserActions | null)[];
		}
		// interface PageState {}
		// interface Platform {}
	}

	interface Window {
		umami?: {
			identify: (id?: string | null | Record<string, unknown>) => Promise<string>;
			track: (...args: unknown[]) => Promise<string>;
		};
	}
}

export {};
