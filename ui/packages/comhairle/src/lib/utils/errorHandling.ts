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

type FetchErr = { status: number; message: string };
export async function tryFetch<T extends Response>(
	fn: () => Promise<T>
): Promise<Result<'ok', T, FetchErr>> {
	try {
		const result = await fn();
		if (!result.ok) {
			const body = await result.json();
			const errMessage: string | undefined = result.status >= 500 ? ERROR_500 : body.err;
			return {
				ok: null,
				err: { status: result.status, message: errMessage ?? JSON.stringify(body) }
			};
		}
		return { ok: result, err: null };
	} catch (e) {
		return { ok: null, err: { status: -1, message: JSON.stringify(e) } };
	}
}
