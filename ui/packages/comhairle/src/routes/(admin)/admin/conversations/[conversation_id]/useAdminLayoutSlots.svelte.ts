import { getContext } from 'svelte';
import type { Snippet } from 'svelte';
import type { AdminPageSlots, BreadcrumbCrumb } from './slotTypes';

interface UseAdminLayoutSlotsOptions {
	breadcrumbs?: BreadcrumbCrumb[] | (() => BreadcrumbCrumb[]) | null;
	title?: Snippet | null;
}

export function useAdminLayoutSlots(options: UseAdminLayoutSlotsOptions = {}): AdminPageSlots {
	const layoutSlots = getContext<AdminPageSlots>('adminLayoutSlots');

	$effect(() => {
		if (options.breadcrumbs) {
			const trail =
				typeof options.breadcrumbs === 'function'
					? options.breadcrumbs()
					: options.breadcrumbs;
			layoutSlots.breadcrumbTrail(trail);
		}
		if (options.title) {
			layoutSlots.titleContent(options.title);
		}

		return () => {
			if (options.breadcrumbs) {
				layoutSlots.clearBreadcrumbTrail();
			}
			if (options.title) {
				layoutSlots.clearTitleContent();
			}
		};
	});

	return layoutSlots;
}
