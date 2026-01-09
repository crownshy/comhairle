import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		knowledgebaseId: params.knowledgebase_id
	};
};
