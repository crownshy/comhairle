type LinkedStatement = { id: string; original_statement_id?: string | null };

/**
 * The statement the participant actually wrote, following edits and splits back to the start.
 * Returns the statement itself when it was never edited.
 */
export function participantOriginal<Statement extends LinkedStatement>(
	statement: Statement,
	statements: Statement[]
): Statement {
	const byId = new Map(statements.map((s) => [s.id, s]));
	const seen = new Set<string>();
	let current = statement;
	while (current.original_statement_id && !seen.has(current.id)) {
		seen.add(current.id);
		const parent = byId.get(current.original_statement_id);
		if (!parent) break;
		current = parent;
	}
	return current;
}
