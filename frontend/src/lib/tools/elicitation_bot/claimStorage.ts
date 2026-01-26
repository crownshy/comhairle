import type { ExtractedClaim } from './types';

export interface ClaimModification {
	editedClaims: Record<string, string>;
	addedClaims: ExtractedClaim[];
	removedClaimIds: Set<string>;
	approvedClaimIds: Set<string>;
}

const STORAGE_KEY_PREFIX = 'elicitation_claims_';

function getStorageKey(botId: string, conversationId: string): string {
	return `${STORAGE_KEY_PREFIX}${botId}_${conversationId}`;
}

export function loadClaimModifications(botId: string, conversationId: string): ClaimModification {
	if (typeof window === 'undefined') {
		return createEmptyModifications();
	}

	try {
		const key = getStorageKey(botId, conversationId);
		const stored = localStorage.getItem(key);
		if (!stored) {
			return createEmptyModifications();
		}

		const parsed = JSON.parse(stored);
		return {
			editedClaims: parsed.editedClaims || {},
			addedClaims: parsed.addedClaims || [],
			removedClaimIds: new Set(parsed.removedClaimIds || []),
			approvedClaimIds: new Set(parsed.approvedClaimIds || [])
		};
	} catch (e) {
		console.error('Failed to load claim modifications:', e);
		return createEmptyModifications();
	}
}

export function saveClaimModifications(
	botId: string,
	conversationId: string,
	modifications: ClaimModification
): void {
	if (typeof window === 'undefined') return;

	try {
		const key = getStorageKey(botId, conversationId);
		const toStore = {
			editedClaims: modifications.editedClaims,
			addedClaims: modifications.addedClaims,
			removedClaimIds: Array.from(modifications.removedClaimIds),
			approvedClaimIds: Array.from(modifications.approvedClaimIds)
		};
		localStorage.setItem(key, JSON.stringify(toStore));
	} catch (e) {
		console.error('Failed to save claim modifications:', e);
	}
}

export function clearClaimModifications(botId: string, conversationId: string): void {
	if (typeof window === 'undefined') return;

	try {
		const key = getStorageKey(botId, conversationId);
		localStorage.removeItem(key);
	} catch (e) {
		console.error('Failed to clear claim modifications:', e);
	}
}

export function createEmptyModifications(): ClaimModification {
	return {
		editedClaims: {},
		addedClaims: [],
		removedClaimIds: new Set(),
		approvedClaimIds: new Set()
	};
}

export function mergeClaimsWithModifications(
	aiClaims: ExtractedClaim[],
	modifications: ClaimModification
): ExtractedClaim[] {
	const result: ExtractedClaim[] = [];

	for (const claim of aiClaims) {
		if (modifications.removedClaimIds.has(claim.id)) {
			continue;
		}

		const editedContent = modifications.editedClaims[claim.id];
		const isApproved = modifications.approvedClaimIds.has(claim.id);

		result.push({
			...claim,
			content: editedContent ?? claim.content,
			status: isApproved ? 'approved' : claim.status
		});
	}

	for (const addedClaim of modifications.addedClaims) {
		if (!modifications.removedClaimIds.has(addedClaim.id)) {
			result.push(addedClaim);
		}
	}

	return result;
}
