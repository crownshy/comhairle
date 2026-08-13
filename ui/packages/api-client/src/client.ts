import { createApiClient as createApi } from './api';

export const createApiClient = (
	baseUrl: string,
	authToken: string | undefined,
	source: string,
	locale?: string
): ReturnType<typeof createApi> => {
	let api = createApi(baseUrl, {
		axiosConfig: {
			withCredentials: true
		}
	});

	api.axios.interceptors.request.use((config) => {
		if (source === 'server') {
			const cookies: string[] = [];
			if (authToken) {
				cookies.push(`auth-token=${authToken}`);
			}
			if (locale) {
				cookies.push(`COMHAIRLE_LOCALE=${locale}`);
			}
			if (cookies.length > 0) {
				config.headers['Cookie'] = cookies.join('; ');
			}
		}
		return config;
	});

	return api;
};

export const apiClient: ReturnType<typeof createApiClient> = createApiClient(
	'/api',
	undefined,
	'client'
);
