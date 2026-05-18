/**
 * Aggregation utilities for the Prioritization Tool.
 *
 * In the prototype the same set of questions applies to every proposal
 * (poll-wide questions defined in `tool_config`). We compute per-question
 * aggregates per proposal here.
 *
 * The cross-proposal `combination_metric` described in the backend tool
 * config is reserved for a future iteration; the enum is empty in the MVP.
 */

import type { AnswerValue, Poll, Question, Submission } from './types';

/** Map an answer to a numeric value if possible. */
export function numericValue(question: Question, answer: AnswerValue | undefined): number | null {
	if (!answer) return null;
	switch (question.type) {
		case 'likert_scale':
		case 'continuous':
			return answer.kind === 'numeric' ? answer.value : null;
		case 'text':
			return null;
	}
}

/** Map a numeric value into the question's [0,1] range. */
export function normalise(question: Question, value: number): number {
	switch (question.type) {
		case 'likert_scale': {
			const values = question.categories.map((c) => c.value);
			if (values.length === 0) return 0.5;
			const lo = Math.min(...values);
			const hi = Math.max(...values);
			const span = hi - lo;
			return span === 0 ? 0.5 : (value - lo) / span;
		}
		case 'continuous': {
			const span = question.maxValue - question.minValue;
			if (span === 0) return 0.5;
			return (value - question.minValue) / span;
		}
		default:
			return 0.5;
	}
}

export type QuestionAggregate =
	| {
			kind: 'numeric';
			mean: number;
			min: number;
			max: number;
			variance: number;
			n: number;
			histogram: number[];
	  }
	| { kind: 'text'; samples: string[]; n: number };

export type ProposalAggregate = {
	proposalId: string;
	perQuestion: Record<string, QuestionAggregate>;
};

function aggregateQuestion(
	question: Question,
	answers: (AnswerValue | undefined)[]
): QuestionAggregate {
	if (question.type === 'text') {
		const samples = answers
			.filter((a): a is Extract<AnswerValue, { kind: 'text' }> => !!a && a.kind === 'text')
			.map((a) => a.value)
			.filter((v) => v.trim().length > 0);
		return { kind: 'text', samples, n: samples.length };
	}
	const numeric = answers
		.map((a) => numericValue(question, a))
		.filter((v): v is number => v !== null);
	if (numeric.length === 0) {
		return { kind: 'numeric', mean: 0, min: 0, max: 0, variance: 0, n: 0, histogram: [] };
	}
	const mean = numeric.reduce((a, b) => a + b, 0) / numeric.length;
	const variance = numeric.reduce((acc, v) => acc + (v - mean) ** 2, 0) / numeric.length;
	const min = Math.min(...numeric);
	const max = Math.max(...numeric);
	const histogram = histogramFor(question, numeric);
	return { kind: 'numeric', mean, min, max, variance, n: numeric.length, histogram };
}

function histogramFor(question: Question, values: number[]): number[] {
	let lo = 0;
	let hi = 1;
	let buckets = 5;
	if (question.type === 'likert_scale') {
		const cvalues = question.categories.map((c) => c.value);
		buckets = Math.max(1, cvalues.length);
		lo = cvalues.length ? Math.min(...cvalues) : 0;
		hi = cvalues.length ? Math.max(...cvalues) : 1;
	} else if (question.type === 'continuous') {
		const range = Math.abs(question.maxValue - question.minValue);
		buckets = Math.max(2, Math.min(10, Math.round(range) + 1));
		lo = question.minValue;
		hi = question.maxValue;
	}
	const out = new Array(buckets).fill(0);
	const span = hi - lo;
	for (const v of values) {
		if (span === 0) {
			out[0]++;
			continue;
		}
		const t = (v - lo) / span;
		const idx = Math.min(buckets - 1, Math.max(0, Math.floor(t * buckets)));
		out[idx]++;
	}
	return out;
}

/** Aggregate every proposal in the poll. Questions are poll-wide. */
export function aggregatePoll(poll: Poll, submissions: Submission[]): ProposalAggregate[] {
	const questions = poll.toolConfig.questions;
	return poll.proposals.map((proposal) => {
		const perQuestion: Record<string, QuestionAggregate> = {};
		for (const q of questions) {
			const answers = submissions.map((s) => s.byProposal[proposal.id]?.[q.id]);
			perQuestion[q.id] = aggregateQuestion(q, answers);
		}
		return { proposalId: proposal.id, perQuestion };
	});
}
