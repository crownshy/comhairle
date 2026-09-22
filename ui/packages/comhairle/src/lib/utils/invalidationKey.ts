type InvalidationKey = `${string}:${string}`;
type InputKey =
	| 'user'
	| 'admin/conversations'
	| 'admin/conversation'
	| 'admin/conversation/report'
	| 'admin/conversation/workflow'
	| 'admin/conversation/invites'
	| 'admin/conversation/documents'
	| 'admin/conversation/events'
	| 'admin/event'
	| 'admin/workflow-steps'
	| 'admin/knowledge-base/documents'
	| 'admin/email-template-config'
	| 'admin/documents'
	| 'public/conversation'
	| 'public/event'
	| 'public/documents'
	| 'public/notifications'
	| 'public/participation'
	| 'public/workflow-steps';

export function key(k: InputKey): InvalidationKey {
	return `app:${k}`;
}
