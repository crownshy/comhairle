import { getContext, setContext } from 'svelte';

// The Find out more drawer renders in the workflow layout but its phone trigger sits in the
// NavBar, which the public layout renders above it, so the two share the open state here.
class SupportDrawerState {
	open = $state(false);
}

export type { SupportDrawerState };

const SYMBOL_KEY = 'comhairle-support-drawer';

export function setSupportDrawer(): SupportDrawerState {
	return setContext(Symbol.for(SYMBOL_KEY), new SupportDrawerState());
}

export function useSupportDrawer(): SupportDrawerState {
	const context = getContext<SupportDrawerState | undefined>(Symbol.for(SYMBOL_KEY));
	if (!context) {
		throw new Error('useSupportDrawer must be used within the (public) layout');
	}
	return context;
}
