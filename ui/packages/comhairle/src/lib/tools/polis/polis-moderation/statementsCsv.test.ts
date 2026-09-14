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
	moderation_reason: 'Reworded/split by moderator into 1 statement(s)'
});
const statements = [
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
	statement({ polis_statement_id: 4, statement_text: 'More cycle lanes', is_seed: true })
];
const labels = ['Advertising or campaigning', 'Duplicate'];

const HEADER =
	'"statement_id","statement_text","moderation_status","is_seed","reject_reason","reject_note","edited_from_statement_id"';

describe('buildStatementsCsv', () => {
	it('exports every statement in statement id order', () => {
		expect(buildStatementsCsv(statements, 'all', labels).split('\n')).toEqual([
			HEADER,
			'"1","Vote for me","rejected","false","Advertising or campaigning","leaflet link",""',
			'"2","Buses are late and trains are dirty","rejected","false","","Reworded/split by moderator into 1 statement(s)",""',
			'"3","Buses are late","accepted","false","","","2"',
			'"4","More cycle lanes","pending","true","","",""'
		]);
	});

	it('limits rows to one status but still resolves lineage from the full list', () => {
		expect(buildStatementsCsv(statements, 'accepted', labels).split('\n')).toEqual([
			HEADER,
			'"3","Buses are late","accepted","false","","","2"'
		]);
	});

	it('writes just the header when nothing matches', () => {
		expect(buildStatementsCsv([], 'rejected', labels)).toBe(HEADER);
	});
});
