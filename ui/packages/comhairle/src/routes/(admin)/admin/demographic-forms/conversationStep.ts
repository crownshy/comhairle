import type { DemographicForm } from './demographicPrototypeData';

/**
 * Static helpers for the prototype conversation's "Add demographic step".
 * Nothing here talks to an API. A step inherits its questions and their
 * required flags from the chosen form, and can then override them.
 */

export type VersionPolicy = 'latest' | 'pinned';

export type StepQuestion = {
	questionId: string;
	/** false means the step does not ask this question at all */
	ask: boolean;
	required: boolean;
};

export type DemographicStep = {
	id: string;
	name: string;
	formId: string;
	policy: VersionPolicy;
	/** only set when policy is "pinned" */
	pinnedVersion: number | null;
	allowReturn: boolean;
	mustComplete: boolean;
	questions: StepQuestion[];
};

export type StepSettings = {
	name: string;
	policy: VersionPolicy;
	pinnedVersion: number | null;
	allowReturn: boolean;
	mustComplete: boolean;
};

/** Only published forms can be used by a conversation. */
export function selectableForms(forms: DemographicForm[]): DemographicForm[] {
	return forms.filter((form) => form.status === 'published');
}

/** Published versions a step can be pinned to, newest first. */
export function pinnableVersions(form: DemographicForm): number[] {
	return Array.from({ length: form.version }, (_, index) => form.version - index);
}

function inheritFromForm(form: DemographicForm): StepQuestion[] {
	return form.questions.map((question) => ({
		questionId: question.questionId,
		ask: true,
		required: question.required
	}));
}

/** Every question starts asked, with the form's own required flag. */
export function createStep(
	id: string,
	form: DemographicForm,
	settings: StepSettings
): DemographicStep {
	return {
		id,
		name: settings.name.trim() || form.name,
		formId: form.id,
		policy: settings.policy,
		pinnedVersion: settings.policy === 'pinned' ? settings.pinnedVersion : null,
		allowReturn: settings.allowReturn,
		mustComplete: settings.mustComplete,
		questions: inheritFromForm(form)
	};
}

export function versionUsedByStep(step: DemographicStep, form: DemographicForm): number {
	return step.policy === 'pinned' && step.pinnedVersion !== null
		? step.pinnedVersion
		: form.version;
}

export function versionLabel(step: DemographicStep, form: DemographicForm): string {
	return step.policy === 'pinned'
		? `Pinned to v${versionUsedByStep(step, form)}`
		: `Follows latest (v${form.version})`;
}

/** True when the step differs from the form for this question. */
export function isOverridden(step: StepQuestion, form: DemographicForm): boolean {
	const original = form.questions.find((question) => question.questionId === step.questionId);
	if (!original) return false;
	return !step.ask || step.required !== original.required;
}

export function overrideCount(step: DemographicStep, form: DemographicForm): number {
	return step.questions.filter((question) => isOverridden(question, form)).length;
}

export function askedCount(step: DemographicStep): number {
	return step.questions.filter((question) => question.ask).length;
}

export function requiredCount(step: DemographicStep): number {
	return step.questions.filter((question) => question.ask && question.required).length;
}

/** Turning a question off also clears its required flag. */
export function setAsk(step: StepQuestion, ask: boolean): StepQuestion {
	return { ...step, ask, required: ask ? step.required : false };
}

export function resetToForm(step: DemographicStep, form: DemographicForm): DemographicStep {
	return { ...step, questions: inheritFromForm(form) };
}
