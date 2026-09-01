export const TOOL_NAME = 'polis';

import PolisEmbed from './PolisEmbed.svelte';
import PolisEmbedSkeleton from './PolisEmbedSkeleton.svelte';
import PolisReport from './PolisReport.svelte';
import PolisModerate from './PolisModerate.svelte';
import PolisManage from './PolisManage.svelte';

export {
	PolisEmbed as UserUI,
	PolisEmbedSkeleton as UserUISkeleton,
	PolisModerate as ModerationUI,
	PolisManage as ManageUI,
	PolisReport as ReportUI
};
