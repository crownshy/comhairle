import { SvelteSet } from 'svelte/reactivity';
import { isAutosaveSource, type AutosaveSource, type TranslationSource } from './translationUtils';

/** Autosave sources rendered on the current page, for `AutosaveLeaveGuard`. */
export const mountedAutosaveSources = new SvelteSet<AutosaveSource>();

/** Tracks a field's source while the field is mounted. Call during component init. */
export function registerAutosaveSource(getSource: () => TranslationSource) {
	$effect(() => {
		const source = getSource();
		if (!isAutosaveSource(source)) return;
		mountedAutosaveSources.add(source);
		return () => mountedAutosaveSources.delete(source);
	});
}
