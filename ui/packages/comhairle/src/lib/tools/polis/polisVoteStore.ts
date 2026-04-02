const STORAGE_PREFIX = 'comhairle-polis-votes';

interface PolisVoteData {
	totalVotes: number;
	hasMetThreshold: boolean;
}

function storageKey(userId: string, polisId: string): string {
	return `${STORAGE_PREFIX}-${userId}-${polisId}`;
}

function load(userId: string, polisId: string): PolisVoteData {
	if (typeof window === 'undefined') return { totalVotes: 0, hasMetThreshold: false };
	try {
		const raw = localStorage.getItem(storageKey(userId, polisId));
		return raw ? JSON.parse(raw) : { totalVotes: 0, hasMetThreshold: false };
	} catch {
		return { totalVotes: 0, hasMetThreshold: false };
	}
}

function save(userId: string, polisId: string, data: PolisVoteData): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(userId, polisId), JSON.stringify(data));
	} catch {
		/* ignore */
	}
}

export function getVoteData(userId: string, polisId: string): PolisVoteData {
	return load(userId, polisId);
}

export function resetVoteCount(userId: string, polisId: string): PolisVoteData {
	const data = load(userId, polisId);
	data.totalVotes = 0;
	save(userId, polisId, data);
	return data;
}

export function incrementVotes(
	userId: string,
	polisId: string,
	requiredVotes: number
): PolisVoteData {
	const data = load(userId, polisId);
	data.totalVotes++;
	if (data.totalVotes >= requiredVotes) {
		data.hasMetThreshold = true;
	}
	save(userId, polisId, data);
	return data;
}
