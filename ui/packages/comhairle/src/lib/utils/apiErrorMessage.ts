/** Reads the `err` message the API puts in its error body, falling back when there is none. */
export function apiErrorMessage(error: unknown, fallback: string): string {
	if (typeof error !== 'object' || error === null || !('response' in error)) return fallback;
	const response = error.response;
	if (typeof response !== 'object' || response === null || !('data' in response)) return fallback;
	const data = response.data;
	if (typeof data !== 'object' || data === null || !('err' in data)) return fallback;
	return typeof data.err === 'string' && data.err.length > 0 ? data.err : fallback;
}
