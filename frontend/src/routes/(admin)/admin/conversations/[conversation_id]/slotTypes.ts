import type { Snippet } from "svelte";

export interface AdminPageSlots {
	breadcrumbContent: (snippet: Snippet | null) => void;
	titleContent: (snippet: Snippet | null) => void;
	clearTitleContent: () => void;
	clearBreadcrumbContent: () => void;
}
