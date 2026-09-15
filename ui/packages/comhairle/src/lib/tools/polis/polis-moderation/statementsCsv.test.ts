import { describe, expect, it } from 'vitest';
import type { PolisStatementAux } from '@crownshy/api-client/api';
import { buildStatementsCsv } from './statementsCsv';

function statement(overrides: Partial<PolisStatementAux>): PolisStatementAux {
	return {
		created_at: '2026-09-01T10:00:00Z',
		updated_at: '2026-09-01T10:00:00Z',
		id: crypto.randomUUID(),
		is_seed: false,
		moderation_reason: null,
		moderation_status: 'pending',
		original_statement_id: null,
		polis_conversation_id: 'abc',
		polis_statement_id: 0,
		statement_text: '',
		themes: [],
		user_id: null,
		visible_statement_when_submitted: null,
		workflow_step_id: crypto.randomUUID(),
		zid: 1,
		...overrides
	};
}

const original = statement({
	polis_statement_id: 2,
	statement_text: 'Buses are late and trains are dirty',
	moderation_status: 'rejected',
	moderation_reason: 'Reworded/split by moderator into 2 statement(s)'
});
const statements = [
	statement({
		polis_statement_id: 5,
		statement_text: 'Trains are dirty',
		moderation_status: 'accepted',
		original_statement_id: original.id
	}),
	original,
	statement({
		polis_statement_id: 3,
		statement_text: 'Buses are late',
		moderation_status: 'accepted',
		original_statement_id: original.id
	}),
	statement({
		polis_statement_id: 1,
		statement_text: 'Vote for me',
		moderation_status: 'rejected',
		moderation_reason: 'Advertising or campaigning: leaflet link'
	}),
	statement({
		polis_statement_id: 4,
		statement_text: 'More cycle lanes',
		is_seed: true,
		themes: ['Transport', 'Cycling']
	})
];
const labels = ['Advertising or campaigning', 'Duplicate'];

const HEADER =
	'"statement_id","created_at","statement_text","moderation_status","is_seed","themes","reject_reason","reject_note","edited_from_statement_id","replaced_by_statement_ids"';

describe('buildStatementsCsv', () => {
	it('exports every statement in statement id order, with lineage both ways', () => {
		expect(buildStatementsCsv(statements, labels).split('\n')).toEqual([
			HEADER,
			'"1","2026-09-01T10:00:00Z","Vote for me","rejected","false","","Advertising or campaigning","leaflet link","",""',
			'"2","2026-09-01T10:00:00Z","Buses are late and trains are dirty","rejected","false","","","Reworded/split by moderator into 2 statement(s)","","3; 5"',
			'"3","2026-09-01T10:00:00Z","Buses are late","accepted","false","","","","2",""',
			'"4","2026-09-01T10:00:00Z","More cycle lanes","pending","true","Transport; Cycling","","","",""',
			'"5","2026-09-01T10:00:00Z","Trains are dirty","accepted","false","","","","2",""'
		]);
	});

	it('writes just the header when there are no statements', () => {
		expect(buildStatementsCsv([], labels)).toBe(HEADER);
	});
});
