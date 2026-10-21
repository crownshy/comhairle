const STORAGE_PREFIX = 'comhairle-polis-votes';

interface PolisVoteData {
	/** Every vote this participant has cast on this step, including on other devices. */
	totalVotes: number;
	hasMetThreshold: boolean;
	/** `totalVotes` when they last chose Keep voting. The prompt comes back one round of required votes later. */
	votesAtLastPrompt: number;
}

const EMPTY: PolisVoteData = { totalVotes: 0, hasMetThreshold: false, votesAtLastPrompt: 0 };

// `scopeKey` must be unique per workflow step (and per preview vs live), not per
// Polis poll. Two Polis steps can point at the same poll, so keying by poll id
// lets their vote progress and threshold-met flag bleed across each other.
function storageKey(userId: string, scopeKey: string): string {
	return `${STORAGE_PREFIX}-${userId}-${scopeKey}`;
}

function load(userId: string, scopeKey: string): PolisVoteData {
	if (typeof window === 'undefined') return { ...EMPTY };
	try {
		const raw = localStorage.getItem(storageKey(userId, scopeKey));
		return raw ? { ...EMPTY, ...JSON.parse(raw) } : { ...EMPTY };
	} catch {
		return { ...EMPTY };
	}
}

function save(userId: string, scopeKey: string, data: PolisVoteData): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(userId, scopeKey), JSON.stringify(data));
	} catch {
		/* ignore */
	}
}

export function getVoteData(userId: string, scopeKey: string): PolisVoteData {
	return load(userId, scopeKey);
}

export function startNextVotingRound(userId: string, scopeKey: string): PolisVoteData {
	const data = load(userId, scopeKey);
	data.votesAtLastPrompt = data.totalVotes;
	save(userId, scopeKey, data);
	return data;
}

export function incrementVotes(
	userId: string,
	scopeKey: string,
	requiredVotes: number
): PolisVoteData {
	const data = load(userId, scopeKey);
	data.totalVotes++;
	if (data.totalVotes >= requiredVotes) {
		data.hasMetThreshold = true;
	}
	save(userId, scopeKey, data);
	return data;
}

// Merge an authoritative server vote count (votes this participant has cast on
// any device) into the local progress. The server is the source of truth for
// prior sessions, but the local count can be ahead when Polis has not yet caught
// up with votes cast this session, so we take the higher of the two and never
// regress. Recomputes hasMetThreshold and persists the result.
export function reconcileServerVotes(
	userId: string,
	scopeKey: string,
	serverVotes: number,
	requiredVotes: number
): PolisVoteData {
	const data = load(userId, scopeKey);
	data.totalVotes = Math.max(data.totalVotes, serverVotes);
	if (data.totalVotes >= requiredVotes) {
		data.hasMetThreshold = true;
	}
	save(userId, scopeKey, data);
	return data;
}
