import { HttpStatus } from '$lib/utils/constants';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient, type ApiError } from '@crownshy/api-client/client';

export async function prioritizationInsightsLoader(workflowStepId: string) {
	const result = await tryCatchAsync<ApiError>(() =>
		apiClient.GetPrioritizationInsights({
			queries: { workflow_step_id: workflowStepId }
		})
	);

	console.log(result);

	if (result.err !== null) {
		const message = 'Something went wrong retrieving insights data.';
		if (result.err.status === HttpStatus.UnprocessableContent) {
			return {
				prioritization: {
					insights: null,
					error:
						message + ' Please ensure an alignment question is selected for this step.'
				}
			};
		}
		return { prioritization: { insights: null, error: message } };
	}

	return { prioritization: { insights: result.ok, error: null } };
}
