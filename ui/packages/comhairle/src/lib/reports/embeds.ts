import { POLIS_EMBEDDABLE_COMPONENTS } from './polis/embeddableComponents';

/**
 * Report widgets a facilitator can embed into the conversation report, keyed by the Step's
 * tool type (ADR-0012, ADR-0046). Metadata only, so settings pages can check it without
 * bundling chart components. Adding a tool means one entry here and one in
 * `ReportEmbedLive.svelte`.
 */

export type ReportWidgetMeta<T extends string = string> = {
	type: T;
	label: string;
	description: string;
};

export type ReportEmbedProps = {
	toolStepId: string;
	componentType: string;
};

const REPORT_WIDGETS_BY_TOOL: Record<string, ReportWidgetMeta[]> = {
	polis: POLIS_EMBEDDABLE_COMPONENTS
};

export function reportWidgetsForTool(toolType: string | null | undefined): ReportWidgetMeta[] {
	return (toolType && REPORT_WIDGETS_BY_TOOL[toolType]) || [];
}

export function hasReportWidgets(toolType: string | null | undefined): boolean {
	return reportWidgetsForTool(toolType).length > 0;
}

/** The tool that owns a widget type, as stored on an embed node. */
export function toolTypeForWidget(componentType: string): string | null {
	for (const [toolType, widgets] of Object.entries(REPORT_WIDGETS_BY_TOOL)) {
		if (widgets.some((widget) => widget.type === componentType)) return toolType;
	}
	return null;
}
