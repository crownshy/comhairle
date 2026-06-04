/** Public entry point, following the convention used by other tools (polis,
 * learn, heyform, …). Step pages import `Prioritization.UserUI` / `.ManageUI`
 * and pass the workflow step + conversation in as props. */

import PrioritizationManage from './PrioritizationManage.svelte';
import PrioritizationReport from './PrioritizationReport.svelte';
import PrioritizationUser from './PrioritizationUser.svelte';

export const TOOL_NAME = 'prioritization';

export {
	PrioritizationManage as ManageUI,
	PrioritizationUser as UserUI,
	PrioritizationReport as ReportUI
};

export type * from './types';
