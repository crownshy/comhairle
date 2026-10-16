import { getContext, setContext } from 'svelte';

// The Find out more drawer renders in the workflow layout but its phone trigger sits in the
// NavBar, which the public layout renders above it, so the two share the open state here.
export type SupportTab = 'learningAssistant' | 'faqs' | 'privacyPolicy';

class SupportDrawerState {
	open = $state(false);
	/** A fresh object per request, so asking for the same tab twice still switches to it. */
	tabRequest = $state<{ tab: SupportTab }>();

	openOn(tab: SupportTab) {
		this.tabRequest = { tab };
		this.open = true;
	}
}

export type { SupportDrawerState };

const SYMBOL_KEY = 'comhairle-support-drawer';

/** Create the Find out more drawer state and set it for descendants. */
export function setSupportDrawer(): SupportDrawerState {
	return setContext(Symbol.for(SYMBOL_KEY), new SupportDrawerState());
}

/** Read the Find out more drawer state. Must be called under the `(public)` layout. */
export function useSupportDrawer(): SupportDrawerState {
	const context = getContext<SupportDrawerState | undefined>(Symbol.for(SYMBOL_KEY));
	if (!context) {
		throw new Error('useSupportDrawer must be used within the (public) layout');
	}
	return context;
}
