const STORAGE_PREFIX = 'comhairle-polis-votes';

interface PolisVoteData {
	totalVotes: number;
	pid?: number;
	hasMetThreshold: boolean;
}

function storageKey(workflowStepId: string): string {
	return `${STORAGE_PREFIX}-${workflowStepId}`;
}

function load(workflowStepId: string): PolisVoteData {
	if (typeof window === 'undefined') return { totalVotes: 0, hasMetThreshold: false };
	try {
		const raw = localStorage.getItem(storageKey(workflowStepId));
		return raw ? JSON.parse(raw) : { totalVotes: 0, hasMetThreshold: false };
	} catch {
		return { totalVotes: 0, hasMetThreshold: false };
	}
}

function save(workflowStepId: string, data: PolisVoteData): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(storageKey(workflowStepId), JSON.stringify(data));
	} catch {
		/* ignore */
	}
}

export function getVoteData(workflowStepId: string): PolisVoteData {
	return load(workflowStepId);
}

export function incrementVotes(workflowStepId: string, requiredVotes: number): PolisVoteData {
	const data = load(workflowStepId);
	data.totalVotes++;
	if (data.totalVotes >= requiredVotes) {
		data.hasMetThreshold = true;
	}
	save(workflowStepId, data);
	return data;
}

export function savePid(workflowStepId: string, pid: number): void {
	const data = load(workflowStepId);
	data.pid = pid;
	save(workflowStepId, data);
}

export function getSavedPid(workflowStepId: string): number | undefined {
	return load(workflowStepId).pid;
}
