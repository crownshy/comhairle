import type { DemographicsResponse } from '@crownshy/api-client/api';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { apiClient } from '@crownshy/api-client/client';

type Action = 'UPDATE' | 'CREATE' | 'DELETE';

function getAction(
	newValue: string | undefined,
	currentResponse: DemographicsResponse | undefined
): Action | undefined {
	const newValueValid = newValue !== null && newValue !== '' && newValue !== undefined;

	if (newValueValid && !currentResponse) {
		return 'CREATE';
	}
	if (!newValueValid && currentResponse) {
		return 'DELETE';
	}
	if (newValueValid && currentResponse && currentResponse.value !== newValue.trim()) {
		return 'UPDATE';
	}
	return undefined;
}

type UpdateResponse = {
	questionSlug: string;
	userId: string;
	newValue: string | undefined;
	currentResponse: DemographicsResponse | undefined;
};

async function updateResponse(params: UpdateResponse) {
	const action = getAction(params.newValue, params.currentResponse);

	if (action === undefined) {
		return undefined;
	}

	return tryCatchAsync(async () => {
		switch (action) {
			case 'CREATE':
				return apiClient.CreateDemographicsResponse({
					questionSlug: params.questionSlug,
					userId: params.userId,
					value: params.newValue?.trim() ?? ''
				});

			case 'DELETE':
				return apiClient.DeleteDemographicsResponse(undefined, {
					params: { question_slug: params.questionSlug, user_id: params.userId }
				});

			case 'UPDATE':
				return apiClient.UpdateDemographicsResponse(
					{ value: params.newValue?.trim() ?? '' },
					{ params: { question_slug: params.questionSlug, user_id: params.userId } }
				);
		}
	});
}

const Demographics = {
	getAction,
	updateResponse
};

export default Demographics;
