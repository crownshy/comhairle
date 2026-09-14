import type { PolisStatementAux } from '@crownshy/api-client/api';
import { splitReason } from '$lib/moderation/moderationPolicy';
import { toCsv } from '$lib/utils/csv';

export type StatementExportScope = 'all' | 'accepted' | 'rejected' | 'pending';

/**
 * The moderated statements of a Polis step as CSV, one row per statement in statement id
 * order. The stored `moderation_reason` is split back into reason and note columns
 * (ADR-0015 stores them as one string) so rejections can be counted by reason in a
 * spreadsheet. `reasonLabels` are the labels the reason could have come from.
 */
export function buildStatementsCsv(
	statements: PolisStatementAux[],
	scope: StatementExportScope,
	reasonLabels: string[]
): string {
	// Resolved from the full list: a rejected original and its accepted replacement land in
	// different scopes.
	const polisIdByAuxId = new Map(statements.map((s) => [s.id, s.polis_statement_id]));

	const rows = statements
		.filter((s) => scope === 'all' || s.moderation_status === scope)
		.sort((a, b) => a.polis_statement_id - b.polis_statement_id)
		.map((s) => {
			const { label, note } = splitReason(s.moderation_reason, reasonLabels);
			return [
				s.polis_statement_id,
				s.statement_text,
				s.moderation_status,
				s.is_seed,
				label,
				note,
				s.original_statement_id ? (polisIdByAuxId.get(s.original_statement_id) ?? '') : ''
			];
		});

	return toCsv([
		[
			'statement_id',
			'statement_text',
			'moderation_status',
			'is_seed',
			'reject_reason',
			'reject_note',
			'edited_from_statement_id'
		],
		...rows
	]);
}
