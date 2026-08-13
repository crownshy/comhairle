import { mount, unmount, flushSync } from 'svelte';
import PolisReportSection from '$lib/reports/polis/PolisReportSection.svelte';
import type { PolisEmbeddableComponentType } from '$lib/reports/polis/embeddableComponents';
import type { PolisReportData } from '$lib/tools/polis/reportTypes';
import type { PolisStatementAux } from '@crownshy/api-client/api';

/**
 * Freeze a Polis section block to an HTML snapshot (ADR-0012): mount the real component
 * off-screen, let it render, read its markup, unmount. The returned HTML is what gets
 * stored on the embed node and rendered everywhere. All the report components are pure
 * Tailwind (no scoped `<style>`), so the class-based styling survives the innerHTML read.
 *
 * Runs in the browser only (it needs the DOM); call it from the embed dialog, not in SSR.
 */
export function freezePolisComponent(
	componentType: PolisEmbeddableComponentType,
	reportData: PolisReportData | null,
	statementAux: PolisStatementAux[]
): string {
	const host = document.createElement('div');
	// Off-screen but laid out, so anything that reads geometry still gets sane values.
	host.style.position = 'fixed';
	host.style.left = '-10000px';
	host.style.top = '0';
	host.style.width = '900px';
	document.body.appendChild(host);

	let html = '';
	try {
		const component = mount(PolisReportSection, {
			target: host,
			props: { componentType, reportData, statementAux }
		});
		flushSync();
		html = host.innerHTML;
		unmount(component);
	} finally {
		host.remove();
	}
	return html;
}
