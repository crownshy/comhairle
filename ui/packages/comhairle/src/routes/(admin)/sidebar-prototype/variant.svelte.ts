// PROTOTYPE, delete when the sidebar question is settled. See NOTES.md.
import { getContext, setContext } from 'svelte';

export const SIDEBAR_VARIANTS = [
	{ key: 'current', name: 'Current: conversation list' },
	{ key: 'drill-in', name: 'Drill-in: sidebar becomes the conversation' },
	{ key: 'switcher', name: 'Switcher: pick a conversation up top' },
	{ key: 'rail', name: 'Icon rail + conversation panel' },
	{ key: 'journey', name: 'Journey: lifecycle stages' }
] as const;

export type SidebarVariantKey = (typeof SIDEBAR_VARIANTS)[number]['key'];

// A cookie, not sessionStorage, so the server renders the chosen variant on first paint.
export const SIDEBAR_VARIANT_COOKIE = 'sidebar_prototype_variant';

export function isSidebarVariant(value: string | null | undefined): value is SidebarVariantKey {
	return SIDEBAR_VARIANTS.some((variant) => variant.key === value);
}

type SidebarVariantState = {
	readonly current: SidebarVariantKey;
	set: (key: SidebarVariantKey) => void;
};

const KEY = Symbol('sidebar-prototype-variant');

export function setSidebarVariant(state: SidebarVariantState) {
	return setContext(KEY, state);
}

export function useSidebarVariant(): SidebarVariantState | undefined {
	return getContext<SidebarVariantState | undefined>(KEY);
}
