import type { Snippet } from 'svelte';

export type BreadcrumbCrumb = {
	label: string;
	href?: string;
};

export interface AdminPageSlots {
	breadcrumbTrail: (trail: BreadcrumbCrumb[] | null) => void;
	titleContent: (snippet: Snippet | null) => void;
	clearTitleContent: () => void;
	clearBreadcrumbTrail: () => void;
}
