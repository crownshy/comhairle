import type { Component } from 'svelte';
import { tryCatchAsync } from '$lib/utils/errorHandling';

type Loaded<Data, Props extends Record<string, unknown>> = {
	data: Data;
	Section: Component<Props>;
};

/**
 * Loads a live report embed's data and its section component together, and again whenever a
 * prop that `load` reads changes. Call it while a component initialises.
 *
 * Pass the section component as a dynamic import: the editor imports every tool's embed, so
 * the chart library then loads only once an embed shows. It also reads matchMedia on import,
 * which unit tests of the editor don't have.
 *
 * @param load Reads the embed's props before its first `await`, so they are tracked.
 */
export function loadEmbed<Data, Props extends Record<string, unknown>>(
	load: () => [Promise<Data>, Promise<Component<Props>>]
): { readonly loading: boolean; readonly loaded: Loaded<Data, Props> | null } {
	let loading = $state(true);
	let loaded = $state.raw<Loaded<Data, Props> | null>(null);

	$effect(() => {
		const [data, section] = load();
		let cancelled = false;
		loading = true;
		loaded = null;

		(async () => {
			const result = await tryCatchAsync(async () => {
				const [loadedData, Section] = await Promise.all([data, section]);
				return { data: loadedData, Section };
			});
			if (cancelled) return;
			loaded = result.err === null ? result.ok : null;
			loading = false;
		})();

		return () => {
			cancelled = true;
		};
	});

	return {
		get loading() {
			return loading;
		},
		get loaded() {
			return loaded;
		}
	};
}
