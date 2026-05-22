import type { ParticipantClaim, QuestionAnswers, ThinkingSpacePhase } from './types';

/**
 * LocalStorage persistence for an individual participant's progress
 * (answers, accumulated claims, current phase).
 * Keyed by (workflowStepId, conversationId, userId) so the same user can
 * resume mid-flow on refresh.
 *
 * Replace with server-side persistence once the backend supports it
 * (see THINKING_SPACE_TODO.md).
 */

export interface ParticipantState {
	phase: ThinkingSpacePhase;
	answers: QuestionAnswers[];
	claims: ParticipantClaim[];
}

const PREFIX = 'thinking_space_participant_';

function key(workflowStepId: string, conversationId: string, userId: string): string {
	return `${PREFIX}${userId}_${workflowStepId}_${conversationId}`;
}

export function emptyState(): ParticipantState {
	return { phase: 'questions', answers: [], claims: [] };
}

export function loadParticipantState(
	workflowStepId: string,
	conversationId: string,
	userId: string
): ParticipantState {
	if (typeof window === 'undefined') return emptyState();
	try {
		const raw = localStorage.getItem(key(workflowStepId, conversationId, userId));
		if (!raw) return emptyState();
		const parsed = JSON.parse(raw) as Partial<ParticipantState>;
		return {
			phase: parsed.phase ?? 'questions',
			answers: parsed.answers ?? [],
			claims: parsed.claims ?? []
		};
	} catch (e) {
		console.error('thinking_space: failed to load participant state', e);
		return emptyState();
	}
}

export function saveParticipantState(
	workflowStepId: string,
	conversationId: string,
	userId: string,
	state: ParticipantState
): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.setItem(key(workflowStepId, conversationId, userId), JSON.stringify(state));
	} catch (e) {
		console.error('thinking_space: failed to save participant state', e);
	}
}

export function clearParticipantState(
	workflowStepId: string,
	conversationId: string,
	userId: string
): void {
	if (typeof window === 'undefined') return;
	try {
		localStorage.removeItem(key(workflowStepId, conversationId, userId));
	} catch (e) {
		console.error('thinking_space: failed to clear participant state', e);
	}
}
