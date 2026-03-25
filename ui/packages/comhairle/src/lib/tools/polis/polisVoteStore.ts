const STORAGE_PREFIX = 'comhairle-polis-votes';

interface PolisVoteData {
	totalVotes: number;
	hasMetThreshold: boolean;
}

function storageKey(userId: string, workflowStepId: string): string {
	return `${STORAGE_PREFIX}-${userId}-${workflowStepId}`;
}

function load(userId: string, workflowStepId: string): PolisVoteData {
	if (typeof window === 'undefined') return { totalVotes: 0, hasMetThreshold: false };
	try {
		const raw = localStorage.getItem(storageKey(userId, workflowStepId));
		return raw ? JSON.parse(raw) : { totalVotes: 0, hasMetThreshold: false };
	} catch {
		return { totalVotes: 0, hasMetThreshold: false };
	}
}

function save(userId: string, workflowStepId: string, data: PolisVoteData): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(userId, workflowStepId), JSON.stringify(data));
	} catch {
		/* ignore */
	}
}

export function getVoteData(userId: string, workflowStepId: string): PolisVoteData {
	return load(userId, workflowStepId);
}

export function incrementVotes(
	userId: string,
	workflowStepId: string,
	requiredVotes: number
): PolisVoteData {
	const data = load(userId, workflowStepId);
	data.totalVotes++;
	if (data.totalVotes >= requiredVotes) {
		data.hasMetThreshold = true;
	}
	save(userId, workflowStepId, data);
	return data;
}
