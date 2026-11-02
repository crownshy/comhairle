import { key } from '$lib/utils/invalidationKey';
import { loadPolisStatements } from '$lib/tools/polis/loadStatements';
import type { PageLoad } from './$types';

// Polis Setup lists the step's seed statements, so it loads the statements here.
export const load: PageLoad = async ({ parent, depends }) => {
	depends(key('admin/polis/statements'));
	const { api, toolConfig, step_id } = await parent();
	if (toolConfig?.type !== 'polis') return { statements: [], statementsFailed: false };
	return loadPolisStatements(api, step_id);
};
