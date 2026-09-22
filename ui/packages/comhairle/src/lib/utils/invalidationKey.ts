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
	| 'public/conversation'
	| 'public/event'
	| 'knowledge-base/documents'
	| 'notifications'
	| 'documents'
	| 'participation'
	| 'email-template-config'
	| 'workflow-steps';

export function key(k: InputKey): InvalidationKey {
	return `app:${k}`;
}
