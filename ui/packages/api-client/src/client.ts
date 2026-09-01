import { createApiClient as createApi } from './api';

// Server-side calls hairpin out to the public origin (the root layout builds the client
// with `url.origin + '/api'`), so a stalled ingress leaves a load function waiting forever
// and nginx answers the browser with a 502. Axios has no timeout of its own. Long enough for
// the slowest read a load function makes, short enough to fail before nginx gives up.
const SERVER_REQUEST_TIMEOUT_MS = 30_000;

export const createApiClient = (
	baseUrl: string,
	authToken: string | undefined,
	source: string,
	locale?: string
): ReturnType<typeof createApi> => {
	let api = createApi(baseUrl, {
		axiosConfig: {
			withCredentials: true,
			// Browser calls stay uncapped: report generation, thinking space summaries and audio
			// processing are LLM-backed and legitimately run for minutes.
			...(source === 'server' ? { timeout: SERVER_REQUEST_TIMEOUT_MS } : {})
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
