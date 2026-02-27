/**
 * A reusable loading state wrapper for async operations.
 * Automatically manages loading state around async function execution.
 *
 * NOTE: Do NOT destructure `loading` - access it via `loader.loading` to maintain reactivity.
 */
export function useLoading() {
	let loading = $state(false);

	async function run<T>(fn: () => Promise<T>): Promise<T | undefined> {
		loading = true;
		try {
			return await fn();
		} finally {
			loading = false;
		}
	}

	return {
		get loading() {
			return loading;
		},
		run
	};
}
