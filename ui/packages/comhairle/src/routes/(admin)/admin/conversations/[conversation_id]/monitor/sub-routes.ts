export const MONITOR_PAGES = [
	{ route: '/(admin)/admin/conversations/[conversation_id]/monitor', label: 'At a glance' },
	{
		route: '/(admin)/admin/conversations/[conversation_id]/monitor/pulse',
		label: 'Live pulse'
	},
	{
		route: '/(admin)/admin/conversations/[conversation_id]/monitor/journey',
		label: 'Journey'
	},
	{
		route: '/(admin)/admin/conversations/[conversation_id]/monitor/recruitment',
		label: 'Recruitment'
	}
] as const;

export type MonitorRoute = (typeof MONITOR_PAGES)[number]['route'];
