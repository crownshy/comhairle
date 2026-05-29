/**
 * MOCK — placeholder until the backend summary endpoint exists.
 *
 * The real implementation will POST to something like
 * `/api/tools/thinking_space/summary` with the workflow step id and the full
 * Q&A history, and stream back a coherent 2nd-person paragraph from a
 * dedicated RAGFlow agent. See THINKING_SPACE_SUMMARY.md for the open
 * backend questions.
 */

import type { QuestionConfig, QuestionAnswers } from './types';

export interface FetchSummaryParams {
	workflowStepId: string;
	topic: string;
	questions: QuestionConfig[];
	answers: QuestionAnswers[];
}

const MOCK_DELAY_MS = 1500;

const MOCK_SUMMARY = `You believe this topic genuinely matters and that the way it is handled has real consequences for people in your community. Across your answers, a clear thread runs through what you shared: you care about the practical impact, not just the principle.

You feel strongly that the people most affected should have more of a say, and you are uncertain about some of the trade-offs — particularly around where resources should go and who decides. You would welcome the chance to hear from others with direct experience before settling firmly on a position.

Going into the deliberation, you want to keep an open mind on the details while holding on to the values you have surfaced here.`;

export async function fetchSummary(_params: FetchSummaryParams): Promise<string> {
	// TODO: replace with real endpoint. See THINKING_SPACE_SUMMARY.md.
	await new Promise((resolve) => setTimeout(resolve, MOCK_DELAY_MS));
	return MOCK_SUMMARY;
}

export async function saveSummary(params: {
	workflowStepId: string;
	summary: string;
}): Promise<void> {
	// TODO: replace with real endpoint. See THINKING_SPACE_SUMMARY.md.
	// Demo-only: persist to localStorage so revisits can hydrate without a
	// backend round-trip and skip the AI generation step.
	await new Promise((resolve) => setTimeout(resolve, 300));
	if (typeof localStorage === 'undefined') return;
	localStorage.setItem(storageKey(params.workflowStepId), params.summary);
}

/**
 * Hydrate a previously submitted summary on revisit. Returns null when none
 * has been saved yet (first visit, or backend endpoint not yet wired up).
 *
 * When this returns a string, the Summary screen should render it directly
 * without calling the AI generation endpoint again.
 */
export async function hydrateSummary(params: { workflowStepId: string }): Promise<string | null> {
	// TODO: replace with real GET endpoint. See THINKING_SPACE_SUMMARY.md.
	// Demo-only: read from localStorage (paired with saveSummary above).
	if (typeof localStorage === 'undefined') return null;
	return localStorage.getItem(storageKey(params.workflowStepId));
}

function storageKey(workflowStepId: string): string {
	return `thinking_space:summary:${workflowStepId}`;
}
