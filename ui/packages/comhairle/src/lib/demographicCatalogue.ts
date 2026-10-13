import type { FormDto, QuestionDto } from '@crownshy/api-client/api';
import { apiClient } from '@crownshy/api-client/client';
import type { DemographicSelection } from '$lib/createWorkflowStep';
import { tryCatchAsync, type ErrorType, type Result } from '$lib/utils/errorHandling';

/** A question or form a conversation step can reference, with a short line for the picker. */
export type PublishedDemographicItem = DemographicSelection & {
	detail: string;
	/** The published version a step pins to when it is set to a fixed version. */
	versionId: string;
};

export type PickerLoad =
	| { status: 'loading' }
	| { status: 'ready'; items: PublishedDemographicItem[] }
	| { status: 'error' };

/**
 * Only items with a published version and no archive date can back a step: the API rejects
 * steps that point at a draft-only or archived question or form.
 */
function publishedQuestionItem(question: QuestionDto): PublishedDemographicItem | undefined {
	const version = question.latest_published;
	if (!version || question.archived_at) return undefined;
	return {
		id: question.id,
		name: version.text,
		detail: `Version ${version.version_number}`,
		versionId: version.id
	};
}

function publishedFormItem(form: FormDto): PublishedDemographicItem | undefined {
	const version = form.latest_published;
	if (!version || form.archived_at) return undefined;
	const questionCount = version.entries.length;
	return {
		id: form.id,
		name: version.name,
		detail: `${questionCount} ${questionCount === 1 ? 'question' : 'questions'} · Version ${version.version_number}`,
		versionId: version.id
	};
}

function byName(a: PublishedDemographicItem, b: PublishedDemographicItem): number {
	return a.name.localeCompare(b.name);
}

export async function listPublishedDemographicQuestions(): Promise<
	Result<'ok', PublishedDemographicItem[], ErrorType>
> {
	return tryCatchAsync(async () => {
		const questions = await apiClient.ListDemographicQuestions();
		return questions.flatMap((question) => publishedQuestionItem(question) ?? []).sort(byName);
	});
}

export async function listPublishedDemographicForms(): Promise<
	Result<'ok', PublishedDemographicItem[], ErrorType>
> {
	return tryCatchAsync(async () => {
		const forms = await apiClient.ListDemographicForms();
		return forms.flatMap((form) => publishedFormItem(form) ?? []).sort(byName);
	});
}
