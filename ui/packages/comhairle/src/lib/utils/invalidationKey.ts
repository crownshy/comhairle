type InvalidationKey = `${string}:${string}`;
type InputKey =
	| 'admin/conversation'
	| 'admin/conversation/documents'
	| 'admin/conversation/events'
	| 'admin/conversation/invites'
	| 'admin/conversation/meta'
	| 'admin/conversation/permissions'
	| 'admin/conversation/report'
	| 'admin/conversation/workflow'
	| 'admin/conversations'
	| 'admin/documents'
	| 'admin/email-template-config'
	| 'admin/event'
	| 'admin/knowledge-base/documents'
	| 'admin/workflow-steps'
	| 'public/conversation'
	| 'public/documents'
	| 'public/event'
	| 'public/notifications'
	| 'public/participation'
	| 'public/workflow-steps'
	| 'user';

export function key(k: InputKey): InvalidationKey {
	return `app:${k}`;
}
