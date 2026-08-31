import { createApiClient as createApi } from './api';

/**
 * Single promise for refresh requests to avoid race conditions if multiple
 * components make requests simultaneously and fail with 401, thus triggering
 * the refresh flow multiple times with the same `refresh-token` cookie. This
 * would fail on any attempt after the first due to token re-use restrictions.
 */
let refreshPromise: Promise<boolean> | null = null;

async function refreshSession(): Promise<boolean> {
	if (!refreshPromise) {
		refreshPromise = fetch('/api/auth/refresh', {
			method: 'POST',
		})
		.then((res) => res.ok)
		.catch(() => false)
		.finally(() => {
				refreshPromise = null;
			})

	}

	return refreshPromise
}

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

	if (source === 'client') {
		api.axios.interceptors.response.use(
			(response) => response,
			async (error) => {
				const originalRequest = error.config;

				if (error.response?.status === 401 &&
					!originalRequest._retried && 
					!originalRequest.url?.includes('/api/auth/refresh')
				) {
					// Prevents infinite loop of retries by axios
					originalRequest._retried = true;

					const refreshed = await refreshSession();
					if (refreshed) {
						return api.axios(originalRequest);
					}
				}

				return Promise.reject(error);
			}
		)
	}

	return api;
};

export const apiClient: ReturnType<typeof createApiClient> = createApiClient(
	'/api',
	undefined,
	'client'
);
