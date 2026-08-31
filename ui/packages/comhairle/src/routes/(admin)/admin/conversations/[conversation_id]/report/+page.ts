import { FullReportDto } from '@crownshy/api-client/api';
import type { PageLoad } from './$types';
import type { EmbeddableStep } from '$lib/components/RichTextEditor/ReportEmbedControls.svelte';
import { key } from '$lib/utils/invalidationKey';
import { tryCatchAsync, type Result } from '$lib/utils/errorHandling';
import { notifications } from '$lib/notifications.svelte';
import { redirect } from '@sveltejs/kit';
import { HttpStatus } from '$lib/utils/constants';
import { resolve } from '$app/paths';
import type { ToolType } from '$lib/tool_meta';

// Tools that have embeddable report components today. Grows as more tools get a
// component set (Thinking Space is next); HeyForm has none yet.
const REPORT_CAPABLE_TOOLS: ToolType[] = ['polis'];

export const load: PageLoad = async ({ parent, depends, params }) => {
	depends(key('conversation/report'));

	const { api, workflows } = await parent();
	const { conversation_id } = params;

	const workflowStepsRequest = tryCatchAsync(() =>
		api.ListConversationWorkflowSteps({
			params: { conversation_id, workflow_id: workflows[0].id }
		})
	);

	const reportRequest: Promise<Result<'ok', FullReportDto, string>> = (async () => {
		// IIFE to try and get the report and if it doesn't exist then make one and return it
		const _report = await tryCatchAsync(() =>
			api.GetReportForConversation({
				params: { conversation_id },
				queries: { withTranslations: true }
			})
		);
		if (_report.ok !== null) {
			return _report;
		}
		return await tryCatchAsync(() =>
			api.GenerateReportForConversation(undefined, {
				params: { conversation_id }
			})
		);
	})();

	const workflowSteps = await workflowStepsRequest;
	const report = await reportRequest;

	if (workflowSteps.err !== null) {
		notifications.addFlash({
			message: 'Failed to retrieve workflow steps, please try again',
			priority: 'ERROR'
		});
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/configure', { conversation_id })
		);
	}

	if (report.err !== null) {
		notifications.addFlash({
			message: 'Failed to retrieve report, please try again',
			priority: 'ERROR'
		});
		redirect(
			HttpStatus.TemporaryRedirect,
			resolve('/(admin)/admin/conversations/[conversation_id]/configure', { conversation_id })
		);
	}

	// Steps offered by the "Embed report component" control, resolved to the shape it needs.
	const reportEmbedSteps: EmbeddableStep[] = (workflowSteps.ok ?? [])
		.filter((step) => {
			const toolType = step.toolConfig?.type;
			return !!toolType && REPORT_CAPABLE_TOOLS.includes(toolType);
		})
		.map((step) => ({
			id: step.id,
			name: step.name,
			toolType: step.toolConfig!.type
		}));

	return { report: report.ok, reportEmbedSteps };
};
