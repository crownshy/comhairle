import type { PolisStatementAux } from '@crownshy/api-client/api';
import { splitReason } from '$lib/moderation/moderationPolicy';
import { toCsv } from '$lib/utils/csv';

/**
 * The moderated statements of a Polis step as CSV, one row per statement in statement id
 * order. The stored `moderation_reason` is split back into reason and note columns
 * (ADR-0015 stores them as one string) so rejections can be counted by reason in a
 * spreadsheet. `reasonLabels` are the labels the reason could have come from.
 */
export function buildStatementsCsv(
	statements: PolisStatementAux[],
	reasonLabels: string[]
): string {
	// Lineage links rows by their Comhairle row id; the file shows Polis statement ids instead.
	const polisStatementIdByRowId = new Map(statements.map((s) => [s.id, s.polis_statement_id]));
	const replacementPolisIdsByRowId = new Map<string, number[]>();
	for (const statement of statements) {
		if (!statement.original_statement_id) continue;
		const replacements = replacementPolisIdsByRowId.get(statement.original_statement_id) ?? [];
		replacements.push(statement.polis_statement_id);
		replacementPolisIdsByRowId.set(statement.original_statement_id, replacements);
	}

	const rows = [...statements]
		.sort((a, b) => a.polis_statement_id - b.polis_statement_id)
		.map((s) => {
			const { label, note } = splitReason(s.moderation_reason, reasonLabels);
			const editedFrom = s.original_statement_id
				? polisStatementIdByRowId.get(s.original_statement_id)
				: undefined;
			const replacedBy = replacementPolisIdsByRowId.get(s.id) ?? [];
			return [
				s.polis_statement_id,
				s.created_at,
				s.statement_text,
				s.moderation_status,
				s.is_seed,
				s.themes.join('; '),
				label,
				note,
				editedFrom ?? '',
				[...replacedBy].sort((a, b) => a - b).join('; ')
			];
		});

	return toCsv([
		[
			'statement_id',
			'created_at',
			'statement_text',
			'moderation_status',
			'is_seed',
			'themes',
			'reject_reason',
			'reject_note',
			'edited_from_statement_id',
			'replaced_by_statement_ids'
		],
		...rows
	]);
}
