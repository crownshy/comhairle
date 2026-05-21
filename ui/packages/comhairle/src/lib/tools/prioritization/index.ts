/** Public entry point following the convention used by other tools (polis, learn, heyform, …). Step pages import `Prioritization.UserUI` or `Prioritization.ManageUI` and pass workflow_step + conversation in. */

import PrioritizationToolWrapper from './PrioritizationToolWrapper.svelte';

export const TOOL_NAME = 'prioritization';

export {
	PrioritizationToolWrapper as Wrapper,
	PrioritizationToolWrapper as UserUI,
	PrioritizationToolWrapper as ManageUI,
	PrioritizationToolWrapper as ReportUI
};

export { createComhairleAdapter } from './adapters/comhairleAdapter';
export { createMemoryAdapter } from './adapters/memoryAdapter';
export type { PrioritizationAdapter } from './adapter';
export type * from './types';
