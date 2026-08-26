type InvalidationKey = `${string}:${string}`;
type InputKey =
	| 'user'
	| 'conversation'
	| 'conversation/report'
	| 'conversation/workflow'
	| 'conversation/invites'
	| 'conversation/documents'
	| 'knowledge-base/documents'
	| 'notifications'
	| 'event'
	| 'documents'
	| 'participation'
	| 'email-template-config'
	| 'workflow-steps';

export function key(k: InputKey): InvalidationKey {
	return `app:${k}`;
}
