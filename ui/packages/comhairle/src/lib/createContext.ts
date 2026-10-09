/**
 * We're on Svelte 5.33 currently
 * Currently a polyfill for a function introduced in Svelte 5.40: https://svelte.dev/docs/svelte/context#:~:text=5%2E40
 * FIX: When we upgrade to Svelte 5.40
 */
import { getContext, setContext } from 'svelte';

function createContext<T>() {
	const id = crypto.randomUUID();
	return [() => getContext<T>(id), (value: T) => setContext(id, value)] as const;
}

export default createContext;
