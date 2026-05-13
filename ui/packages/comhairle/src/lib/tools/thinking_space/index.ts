// Backend still identifies this tool with the `elicitationbot` slug, see
// THINKING_SPACE_TODO.md for the proposed rename + schema work.
export const TOOL_NAME = 'elicitationbot';

import ThinkingSpaceEmbed from './ThinkingSpaceEmbed.svelte';
import ThinkingSpaceManage from './ThinkingSpaceManage.svelte';

export { ThinkingSpaceEmbed as UserUI, ThinkingSpaceManage as ManageUI };

export * from './types';
