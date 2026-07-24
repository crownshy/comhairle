export const TOOL_NAME = 'heyform';

import HeyFormEmbed from './HeyFormEmbed.svelte';
import HeyFormEmbedSkeleton from './HeyFormEmbedSkeleton.svelte';
import HeyFormReport from './HeyFormReport.svelte';
import HeyFormModerate from './HeyFormModerate.svelte';
import HeyFormManage from './HeyFormManage.svelte';

export {
	HeyFormEmbed as UserUI,
	HeyFormEmbedSkeleton as UserUISkeleton,
	HeyFormModerate as ModerationUI,
	HeyFormManage as ManageUI,
	HeyFormReport as ReportUI
};
