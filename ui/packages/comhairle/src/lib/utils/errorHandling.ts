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

type FetchErr = { status: number; message: string };
export async function tryFetch<T extends Response>(
	fn: () => Promise<T>
): Promise<Result<'ok', T, FetchErr>> {
	const response = await fn();
	console.log(response);
	if (!response.ok) {
		const errMessage = await getErrorMessage(response);
		return {
			ok: null,
			err: {
				status: response.status,
				message: errMessage ?? 'Unrecognised error, please contact us'
			}
		};
	}
	return { ok: response, err: null };
}
