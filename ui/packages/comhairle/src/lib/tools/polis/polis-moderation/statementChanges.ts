import { diffWords } from 'diff';

export type StatementChangePart = { text: string; kind: 'kept' | 'added' | 'removed' };

/** Word-level changes from a participant's statement to a moderator's replacement. */
export function statementChanges(original: string, replacement: string): StatementChangePart[] {
	return diffWords(original.trim(), replacement.trim()).map((part) => ({
		text: part.value,
		kind: part.added ? 'added' : part.removed ? 'removed' : 'kept'
	}));
}

/** True when the replacement says exactly what the original said, ignoring outer spaces. */
export function isUnchanged(original: string, replacement: string): boolean {
	return original.trim() === replacement.trim();
}
