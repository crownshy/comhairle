type InvalidationKey = `${string}:${string}`;
type InputKey =
	| 'user'
	| 'conversations'
	| 'conversation'
	| 'conversation/report'
	| 'conversation/workflow'
	| 'conversation/invites'
	| 'conversation/documents'
	| 'conversation/event'
	| 'conversation/events'
	| 'knowledge-base/documents'
	| 'notifications'
	| 'event'
	| 'event/facilitators'
	| 'documents'
	| 'participation'
	| 'email-template-config'
	| 'workflow-steps';

export function key(k: InputKey): InvalidationKey {
	return `app:${k}`;
}
