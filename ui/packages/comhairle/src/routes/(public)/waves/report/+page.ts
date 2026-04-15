import type { PageLoad } from './$types';
import type { PolisReportData } from '$lib/types/report';
import mockData from '$lib/types/report-data.json';

// TODO: Replace mock data with API call:
// const data = await event.fetch(`/api/tools/polis/report_data?workflow_step_id=${stepId}`)
//   .then(r => r.json());

export const load: PageLoad = async () => {
	const reportData: PolisReportData = mockData as PolisReportData;

	return {
		reportData
	};
};
