import { apiClient } from '@crownshy/api-client/client';
import type { PolisStatementAux } from '@crownshy/api-client/api';

export const CONVERSATION_ID = '00000000-0000-4000-8000-000000000001';
export const WORKFLOW_ID = '00000000-0000-4000-8000-000000000002';
export const STEP_ID = '00000000-0000-4000-8000-000000000003';

const SAMPLE: [number, PolisStatementAux['moderation_status'], string][] = [
	[0, 'accepted', 'Shorter queues matter more to me than having more rides to choose from.'],
	[1, 'accepted', 'The park should be easier to get around for people using wheelchairs.'],
	[2, 'accepted', 'Ticket prices should be lower on weekdays when it is less busy.'],
	[
		3,
		'pending',
		'There should be more snack stands, cold drinks, and shaded spots to rest when it gets hot or tiring.'
	],
	[
		4,
		'pending',
		'Getting to go on your favorite rides without waiting forever matters more than planning every hour ahead.'
	]
];

function makeStatement(
	polisId: number,
	status: PolisStatementAux['moderation_status'],
	text: string,
	extra: Partial<PolisStatementAux> = {}
): PolisStatementAux {
	const now = new Date().toISOString();
	return {
		id: crypto.randomUUID(),
		created_at: now,
		updated_at: now,
		is_seed: polisId < 3,
		moderation_status: status,
		moderation_reason: null,
		original_statement_id: null,
		polis_conversation_id: 'lab',
		polis_statement_id: polisId,
		statement_text: text,
		themes: [],
		workflow_step_id: STEP_ID,
		zid: 1,
		...extra
	};
}

// Module state survives `invalidate`, so the page's reload after a decision keeps it.
let statements: PolisStatementAux[] = SAMPLE.map(([id, status, text]) =>
	makeStatement(id, status, text)
);

export function mockStatements() {
	return statements;
}

function moderate(id: string, decision: string, reason: string | null) {
	const status =
		decision === 'accept' ? 'accepted' : decision === 'reject' ? 'rejected' : 'pending';
	const updated = {
		...statements.find((s) => s.id === id)!,
		moderation_status: status,
		moderation_reason: status === 'accepted' ? null : reason
	} as PolisStatementAux;
	statements = statements.map((s) => (s.id === id ? updated : s));
	return updated;
}

function split(id: string, texts: string[]) {
	const original = moderate(id, 'reject', 'Multiple themes');
	let nextId = Math.max(...statements.map((s) => s.polis_statement_id)) + 1;
	const replacements = texts.map((text) =>
		makeStatement(nextId++, 'accepted', text, { original_statement_id: id, is_seed: false })
	);
	statements = [...statements, ...replacements];
	return { original, replacements };
}

function route(method: string, url: string, body: Record<string, unknown>): unknown {
	const moderateOne = url.match(/statement_aux\/([^/]+)\/moderate$/);
	if (moderateOne && !url.endsWith('moderate_batch')) {
		return moderate(moderateOne[1], body.decision as string, body.moderation_reason as string);
	}
	if (url.endsWith('moderate_batch')) {
		const ids = body.ids as string[];
		const succeeded = ids.map((id) =>
			moderate(id, body.decision as string, body.moderation_reason as string)
		);
		return { succeeded, failed: [] };
	}
	const splitOne = url.match(/statement_aux\/([^/]+)\/split$/);
	if (splitOne) return split(splitOne[1], body.replacements as string[]);
	if (url.endsWith('statement_aux/sync')) {
		return { statements, synced: 0, skipped_invalid_xid: 0 };
	}
	// Settings saves (moderator emails) and anything else: accept and echo.
	return method === 'put' ? body : {};
}

type AxiosAdapter = Extract<
	NonNullable<typeof apiClient.axios.defaults.adapter>,
	(...args: never[]) => unknown
>;

const labAdapter: AxiosAdapter = async (config) => {
	const body = typeof config.data === 'string' ? JSON.parse(config.data) : (config.data ?? {});
	await new Promise((resolve) => setTimeout(resolve, 250));
	return {
		data: route(config.method ?? 'get', config.url ?? '', body),
		status: 200,
		statusText: 'OK',
		headers: {},
		config
	};
};

/** Answers the moderation page's API calls in the browser, so the lab runs with no backend. */
export function installMockApi() {
	apiClient.axios.defaults.adapter = labAdapter;
	// Zodios validates responses; the settings echo doesn't match its full schema.
	apiClient.axios.defaults.validateStatus = () => true;
}
