import { getContext } from 'svelte';
import type { Snippet } from 'svelte';
import type { AdminPageSlots } from './slotTypes';

interface UseAdminLayoutSlotsOptions {
	breadcrumbs?: Snippet | null;
	title?: Snippet | null;
}

export function useAdminLayoutSlots(options: UseAdminLayoutSlotsOptions = {}): AdminPageSlots {
	const layoutSlots = getContext<AdminPageSlots>('adminLayoutSlots');

	$effect(() => {
		if (options.breadcrumbs) {
			layoutSlots.breadcrumbContent(options.breadcrumbs);
		}
		if (options.title) {
			layoutSlots.titleContent(options.title);
		}

		return () => {
			if (options.breadcrumbs) {
				layoutSlots.clearBreadcrumbContent();
			}
			if (options.title) {
				layoutSlots.clearTitleContent();
			}
		};
	});

	return layoutSlots;
}
