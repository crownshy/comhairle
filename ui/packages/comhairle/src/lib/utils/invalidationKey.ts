type InvalidationKey = `${string}:${string}`;
type InputKey =
	| 'user'
	| 'conversation'
	| 'conversation/report'
	| 'conversation/workflow'
	| 'notifications'
	| 'event'
	| 'documents'
	| 'participation'
	| 'email-template-config'
	| 'workflow-steps';

export function key(k: InputKey): InvalidationKey {
	return `app:${k}`;
}
