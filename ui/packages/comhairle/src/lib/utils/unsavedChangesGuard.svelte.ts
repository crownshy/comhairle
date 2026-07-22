import { beforeNavigate } from '$app/navigation';
import { onMount } from 'svelte';

/**
 * Warn before leaving the page while there are unsaved changes, e.g. a refresh mid-save on the learn
 * step. Covers a full-page refresh / tab close / external navigation (via the browser's native
 * prompt) and in-app SvelteKit navigation.
 *
 * Call during component initialisation (it uses `onMount` and `beforeNavigate`) and pass a getter
 * for the "dirty" condition, e.g. `guardUnsavedChanges(() => pages.areDirty)`.
 */
export function guardUnsavedChanges(hasUnsavedChanges: () => boolean) {
	// onMount (not $effect): the listener is wired up once and never depends on reactive state. The
	// dirty check runs at event time via the getter, so there is nothing to re-run on state change.
	onMount(() => {
		function handleBeforeUnload(event: BeforeUnloadEvent) {
			if (!hasUnsavedChanges()) return;
			// Triggering the native "you have unsaved changes" prompt: modern browsers key off
			// preventDefault(); older ones need returnValue set. The message itself is not customisable.
			event.preventDefault();
			event.returnValue = '';
		}
		window.addEventListener('beforeunload', handleBeforeUnload);
		return () => window.removeEventListener('beforeunload', handleBeforeUnload);
	});

	beforeNavigate((navigation) => {
		// A full-page unload already gets the native prompt above, so only guard in-app navigation here.
		if (navigation.willUnload || !hasUnsavedChanges()) return;
		if (!confirm('Your changes are still saving. Leave without waiting for them to save?')) {
			navigation.cancel();
		}
	});
}
