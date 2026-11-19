import { getContext, setContext } from 'svelte';

const REPORT_PREVIEW_KEY = Symbol('report-preview');

/**
 * Marks the report page as the preview, so every embed under it loads from the preview
 * endpoints (ADR-0061). Takes a getter so it follows the route as it changes.
 */
export function setReportPreview(isPreview: () => boolean): void {
	setContext(REPORT_PREVIEW_KEY, isPreview);
}

/** Whether the embed sits on the report preview page. False anywhere else. */
export function getReportPreview(): () => boolean {
	return getContext<(() => boolean) | undefined>(REPORT_PREVIEW_KEY) ?? (() => false);
}
