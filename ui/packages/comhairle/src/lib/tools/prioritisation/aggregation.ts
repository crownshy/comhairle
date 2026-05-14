/**
 * Aggregation utilities for the Prioritisation Tool.
 *
 * In the prototype every proposal owns its own question list, so there is
 * no shared importance/agreement axis to combine across proposals. We
 * therefore only compute per-question aggregates per proposal here.
 *
 * The combined importance × agreement metrics described in
 * `documentation/prioritisation-aggregation.md` are deferred until the
 * “shared core + per-proposal extras” model lands (see
 * `documentation/prioritisation-tool-deferred.md` §9).
 */

import type { Poll, Question, Submission, AnswerValue } from './types';

/** Map an answer to a numeric value if possible. */
export function numericValue(question: Question, answer: AnswerValue | undefined): number | null {
	if (!answer) return null;
	switch (question.type) {
		case 'multiple_choice': {
			if (answer.kind !== 'choice') return null;
			const idx = question.choices.findIndex((c) => c.id === answer.choiceId);
			return idx < 0 ? null : idx + 1; // 1..N ordinal
		}
		case 'five_star':
			return answer.kind === 'numeric' ? answer.value : null;
		case 'rating_scale':
			return answer.kind === 'numeric' ? answer.value : null;
		case 'single_line':
		case 'long_text':
			return null;
	}
}

/** Map a numeric value into the question's [0,1] range. */
export function normalise(question: Question, value: number): number {
	switch (question.type) {
		case 'multiple_choice': {
			const n = question.choices.length;
			if (n <= 1) return 0.5;
			return (value - 1) / (n - 1);
		}
		case 'five_star':
			return (value - 1) / 4;
		case 'rating_scale': {
			const span = question.max - question.min;
			if (span === 0) return 0.5;
			return (value - question.min) / span;
		}
		default:
			return 0.5;
	}
}

export type QuestionAggregate =
	| {
			kind: 'choice';
			counts: Record<string, number>;
			percentages: Record<string, number>;
			total: number;
	  }
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
	if (question.type === 'multiple_choice') {
		const counts: Record<string, number> = {};
		question.choices.forEach((c) => (counts[c.id] = 0));
		let total = 0;
		for (const a of answers) {
			if (a && a.kind === 'choice' && counts[a.choiceId] !== undefined) {
				counts[a.choiceId]++;
				total++;
			}
		}
		const percentages: Record<string, number> = {};
		for (const id of Object.keys(counts)) {
			percentages[id] = total ? counts[id] / total : 0;
		}
		return { kind: 'choice', counts, percentages, total };
	}
	if (question.type === 'single_line' || question.type === 'long_text') {
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
	// 10-bucket histogram across the question's natural range
	const histogram = histogramFor(question, numeric);
	return { kind: 'numeric', mean, min, max, variance, n: numeric.length, histogram };
}

function histogramFor(question: Question, values: number[]): number[] {
	let lo = 0;
	let hi = 1;
	let buckets = 5;
	if (question.type === 'multiple_choice') {
		buckets = question.choices.length;
		lo = 1;
		hi = buckets;
	} else if (question.type === 'five_star') {
		buckets = 5;
		lo = 1;
		hi = 5;
	} else if (question.type === 'rating_scale') {
		lo = question.min;
		hi = question.max;
		buckets = Math.min(10, Math.max(2, Math.round(hi - lo)));
	}
	const out = new Array(buckets).fill(0);
	const span = hi - lo;
	for (const v of values) {
		if (span === 0) {
			out[0]++;
			continue;
		}
		const t = (v - lo) / span;
		let idx = Math.min(buckets - 1, Math.max(0, Math.floor(t * buckets)));
		out[idx]++;
	}
	return out;
}

/** Aggregate every proposal in the poll. */
export function aggregatePoll(poll: Poll, submissions: Submission[]): ProposalAggregate[] {
	return poll.proposals.map((proposal) => {
		const perQuestion: Record<string, QuestionAggregate> = {};
		for (const q of proposal.questions) {
			const answers = submissions.map((s) => s.byProposal[proposal.id]?.[q.id]);
			perQuestion[q.id] = aggregateQuestion(q, answers);
		}
		return { proposalId: proposal.id, perQuestion };
	});
}
