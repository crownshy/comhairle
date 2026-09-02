import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, params }) => {
	const { api } = await parent();
	const { conversation_id, event_id } = params;

	const recordings = await api.ListAudioRecordings({
		params: { conversation_id, event_id }
	});

	return {
		recordings
	};
};
