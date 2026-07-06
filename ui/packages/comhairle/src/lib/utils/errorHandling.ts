type ErrorType = string | object;
type Ok<K extends string, T> = Record<K, T> & { err: null };
type Err<K extends string, E extends ErrorType> = Record<K, null> & { err: E };
export type Result<K extends string, T, E extends ErrorType> = Ok<K, T> | Err<K, E>;

export async function tryCatchAsync<T, E extends string>(
	fn: () => Promise<T>
): Promise<Result<'ok', T, E>> {
	try {
		const result = await fn();
		return { ok: result, err: null };
	} catch (err) {
		return { ok: null, err };
	}
}

const NO_INTERNET = 'Network error. Please check your internet connection.';
const ERROR_500 = 'Internal server error. Please try again, and contact us if the issue persists.';

async function getErrorMessage(response: Response): Promise<string | undefined> {
	if (response.status >= 500) {
		return ERROR_500;
	}
	const tryJson = await tryCatchAsync(() => response.json());
	if (tryJson.err === null && tryJson.ok.err) {
		return tryJson.ok.err;
	}
	if (response.statusText) {
		return response.statusText;
	}
	return undefined;
}

export type FetchErr =
	| { id: 'NETWORK_ERROR'; message: typeof NO_INTERNET }
	| { id: 'HTTP_ERROR'; status: number; message: string };
/**
 * Wrapper for fetch that will feed back an error whenever the response is not 200-299, mainly for server-side, client side should use apiClient
 * @param endpoint - URL to fetch
 * @param params - Fetch parameters
 * @param [fetchRef=fetch] - Fetch reference, will use the default fetch, but pass the reference on the backend if used in a +page.server.ts file
 */
export async function tryFetch(
	endpoint: string | URL | Request,
	params: RequestInit | undefined = undefined,
	fetchRef: typeof fetch = fetch
): Promise<Result<'ok', Response, FetchErr>> {
	const response = await tryCatchAsync(() =>
		fetchRef(endpoint, { ...params, credentials: 'include' })
	);
	if (response.err !== null) {
		return { ok: null, err: { id: 'NETWORK_ERROR', message: NO_INTERNET } };
	}
	if (!response.ok.ok) {
		const errMessage = await getErrorMessage(response.ok);
		return {
			ok: null,
			err: {
				id: 'HTTP_ERROR',
				status: response.ok.status,
				message: errMessage ?? 'Unrecognised error, please contact us.'
			}
		};
	}
	return { ok: response.ok, err: null };
}
