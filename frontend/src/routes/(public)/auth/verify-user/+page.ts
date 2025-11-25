import type { PageLoad } from './$types';
import { apiClient } from '$lib/api/client';
import * as m from '$lib/paraglide/messages';

export const load: PageLoad = async ({ url }) => {
	const token = url.searchParams.get('token') ?? '';

	let errorMessage: string | null = null;

	try {
		await apiClient.VerifyEmailToken({ token });
	} catch (e) {
		if (e.response?.status === 409) {
			errorMessage = m.verify_user_conflict_error_body();
		} else {
			errorMessage = m.verify_user_error_body();
		}
	}

	return {
		errorMessage
	};
};
