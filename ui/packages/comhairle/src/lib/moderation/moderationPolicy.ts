/**
 * A conversation's moderation policies (ADR-0038): the reasons a moderator can pick when
 * rejecting a statement. Policies are API records owned by the conversation, and a Polis step
 * points at one through `moderation_policy_id` in its tool config.
 *
 * A chosen label is still stored verbatim in `moderation_reason` as free text (ADR-0015), so
 * editing a policy never rewrites reasons already recorded.
 */
import type {
	ModerationPolicyDto,
	PartialWorkflowStep,
	ToolConfigWithTranslations,
	WorkflowStepWithTranslations
} from '@crownshy/api-client/api';

export interface RejectReason {
	/** The saved reason's id. A save sends it back so the reason is updated in place. */
	id?: string;
	/** Short label shown in the reason picker and stored verbatim in `moderation_reason`. */
	label: string;
	/** What counts under this reason, shown to moderators when they pick it. */
	description?: string;
}

/** Joins a reason's label to the moderator's note in `moderation_reason` (ADR-0015). */
export const REASON_NOTE_SEPARATOR = ': ';

export type RejectReasonLabelProblem = 'blank' | 'contains-separator' | 'duplicate';

/**
 * Why each label can't be saved, or null when it can, in input order. Labels are the picker
 * keys and the stored value, so a repeat (case-insensitive, first one wins) is dropped. A
 * label containing the separator couldn't be split back out of `moderation_reason` on export.
 * The API rejects the same labels.
 */
export function rejectReasonLabelProblems(labels: string[]): (RejectReasonLabelProblem | null)[] {
	const seen = new Set<string>();
	return labels.map((label) => {
		const trimmed = label.trim();
		if (!trimmed) return 'blank';
		if (trimmed.includes(REASON_NOTE_SEPARATOR)) return 'contains-separator';
		const key = trimmed.toLowerCase();
		if (seen.has(key)) return 'duplicate';
		seen.add(key);
		return null;
	});
}

/** Trims every reason and drops the ones `rejectReasonLabelProblems` flags. */
export function cleanRejectReasons(reasons: RejectReason[]): RejectReason[] {
	const problems = rejectReasonLabelProblems(reasons.map((reason) => reason.label));
	const cleaned: RejectReason[] = [];
	reasons.forEach((reason, index) => {
		if (problems[index] !== null) return;
		const label = reason.label.trim();
		const description = reason.description?.trim();
		const withId = reason.id ? { id: reason.id, label } : { label };
		cleaned.push(description ? { ...withId, description } : withId);
	});
	return cleaned;
}

/** A saved policy's reasons, in the shape the editor and the reason picker use. */
export function rejectReasonsFromPolicy(policy: ModerationPolicyDto): RejectReason[] {
	return policy.reasons.map(({ id, label, description }) =>
		description ? { id, label, description } : { id, label }
	);
}

/**
 * The reasons a Polis step offers on reject: its own policy's, otherwise the conversation's
 * first policy's, otherwise the defaults. The middle rule covers a step added after the
 * policy was saved, and a step pointing at a policy that no longer exists.
 */
export function rejectReasonsForStep(
	toolConfig: ToolConfigWithTranslations | null | undefined,
	policies: ModerationPolicyDto[],
	defaultReasons: RejectReason[]
): RejectReason[] {
	const policyId = toolConfig?.type === 'polis' ? toolConfig.moderation_policy_id : null;
	const policy = policies.find((candidate) => candidate.id === policyId) ?? policies[0];
	return policy ? rejectReasonsFromPolicy(policy) : defaultReasons;
}

/**
 * Gives reasons saved for the first time the ids the API assigned, matched by label ignoring
 * case, so the next save updates them in place. A reason renamed while its save was in
 * flight finds no match and is saved as a new reason next time.
 */
export function withSavedReasonIds<T extends RejectReason>(
	reasons: T[],
	saved: RejectReason[]
): T[] {
	const labelKey = (label: string) => label.trim().toLowerCase();
	const heldIds = new Set(reasons.map((reason) => reason.id));
	const unclaimedIds = new Map<string, string>();
	for (const reason of saved) {
		if (reason.id && !heldIds.has(reason.id))
			unclaimedIds.set(labelKey(reason.label), reason.id);
	}

	return reasons.map((reason) => {
		if (reason.id) return reason;
		const id = unclaimedIds.get(labelKey(reason.label));
		if (!id) return reason;
		unclaimedIds.delete(labelKey(reason.label));
		return { ...reason, id };
	});
}

/**
 * The step updates that point every Polis step's preview and live configs at `policyId`, or
 * at no policy when it is null. Configs already pointing there are skipped. `pointedAt` holds
 * where this page last pointed each step, which wins over the loaded config until the steps
 * are loaded again.
 */
export function policyStepUpdates(
	steps: WorkflowStepWithTranslations[],
	policyId: string | null,
	pointedAt: ReadonlyMap<string, string | null> = new Map()
): { stepId: string; body: PartialWorkflowStep }[] {
	const updates: { stepId: string; body: PartialWorkflowStep }[] = [];

	for (const step of steps) {
		const currentPolicyId = (config: { moderation_policy_id?: string | null }) =>
			pointedAt.has(step.id)
				? (pointedAt.get(step.id) ?? null)
				: (config.moderation_policy_id ?? null);

		const body: PartialWorkflowStep = {};
		const preview = step.previewToolConfig;
		if (preview?.type === 'polis' && currentPolicyId(preview) !== policyId) {
			body.preview_tool_config = { ...preview, moderation_policy_id: policyId };
		}
		const live = step.toolConfig;
		if (live?.type === 'polis' && currentPolicyId(live) !== policyId) {
			body.tool_config = { ...live, moderation_policy_id: policyId };
		}

		if (body.preview_tool_config || body.tool_config) updates.push({ stepId: step.id, body });
	}

	return updates;
}

/**
 * Combine the chosen preset label and an optional free-text note into the single
 * string stored in `moderation_reason`. Returns undefined when neither is given,
 * so a reason-less reject stays reason-less.
 */
export function composeReason(label: string | null, note: string): string | undefined {
	const trimmed = note.trim();
	if (label && trimmed) return `${label}${REASON_NOTE_SEPARATOR}${trimmed}`;
	if (label) return label;
	return trimmed || undefined;
}

/**
 * The inverse of composeReason, for export: splits a stored `moderation_reason` back into the
 * label and the note, given the labels it could have come from. Anything that doesn't start
 * with a known label (a renamed label, the split flow's sentence, a note-only reject) comes
 * back whole as the note.
 */
export function splitReason(
	reason: string | null | undefined,
	labels: string[]
): { label: string; note: string } {
	const text = reason?.trim() ?? '';
	if (!text) return { label: '', note: '' };

	// Longest first, so a label that is a prefix of another can't claim its reasons.
	const candidates = [...labels].sort((a, b) => b.length - a.length);
	for (const label of candidates) {
		if (text === label) return { label, note: '' };
		const prefix = `${label}${REASON_NOTE_SEPARATOR}`;
		if (text.startsWith(prefix)) {
			return { label, note: text.slice(prefix.length).trim() };
		}
	}
	return { label: '', note: text };
}
