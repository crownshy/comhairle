import type { PageLoad } from './$types';
import { apiClient } from '@crownshy/api-client/client';
import * as m from '$lib/paraglide/messages';

export const load: PageLoad = async ({ url }) => {
	const token = url.searchParams.get('token') ?? '';

	let errorMessage: string | null = null;
	let status: string | null = null;

	if (token && typeof window !== 'undefined') {
		try {
			await apiClient.VerifyEmailToken({ token });
			status = 'success';
		} catch (e) {
			status = 'error';
			if (e.response?.status === 409) {
				errorMessage = m.verify_user_conflict_error_body();
			} else {
				errorMessage = m.verify_user_error_body();
			}
		}
	}

	return {
		status,
		errorMessage
	};
};
