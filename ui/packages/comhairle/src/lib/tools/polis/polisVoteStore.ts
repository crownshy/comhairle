const STORAGE_PREFIX = 'comhairle-polis-votes';

interface PolisVoteData {
	totalVotes: number;
	hasMetThreshold: boolean;
}

// `scopeKey` must be unique per workflow step (and per preview vs live), not per
// Polis poll. Two Polis steps can point at the same poll, so keying by poll id
// lets their vote progress and threshold-met flag bleed across each other.
function storageKey(userId: string, scopeKey: string): string {
	return `${STORAGE_PREFIX}-${userId}-${scopeKey}`;
}

function load(userId: string, scopeKey: string): PolisVoteData {
	if (typeof window === 'undefined') return { totalVotes: 0, hasMetThreshold: false };
	try {
		const raw = localStorage.getItem(storageKey(userId, scopeKey));
		return raw ? JSON.parse(raw) : { totalVotes: 0, hasMetThreshold: false };
	} catch {
		return { totalVotes: 0, hasMetThreshold: false };
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

export function resetVoteCount(userId: string, scopeKey: string): PolisVoteData {
	const data = load(userId, scopeKey);
	data.totalVotes = 0;
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
