import { createApiClient as createApi } from './api';

// Axios error for failing request
export interface ApiError {
	// Standard
	message: string;
	name: string;
	// Microsoft
	description: string;
	number: number;
	// Mozilla
	fileName: string;
	lineNumber: number;
	columnNumber: number;
	stack: string;
	// Axios
	config: {
		adapter: string[];
		allowAbsoluteUrls: boolean;
		baseURL:string;
		data: undefined
		env: object;
		headers: object;
		maxBodyLength: number;
		maxContentLength: number;
		method: string;
		params: object;
		timeout: number;
		transformRequest: string[]
		transformResponse: string[]
		transitional: { silentJSONParsing: boolean, forcedJSONParsing: boolean, clarifyTimeoutError: boolean  }
		url: string;
		validateStatus: (status: string) => void;
		withCredentials: true
		xsrfCookieName: string;
		xsrfHeaderName: string;
	}
	code: string;
	status: number;
	response: Response;
	request: Request;
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

	return api;
};

export const apiClient: ReturnType<typeof createApiClient> = createApiClient(
	'/api',
	undefined,
	'client'
);
