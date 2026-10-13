import type {
	FormDraftInput,
	FormDto,
	LocationFormat,
	QuestionDraftInput,
	QuestionDto,
	QuestionType2,
	QuestionVersionDto,
	TagDto
} from '@crownshy/api-client/api';
import type { createApiClient } from '@crownshy/api-client/client';
import { tryCatchAsync, type ErrorType } from '$lib/utils/errorHandling';
import {
	isChoiceKind,
	type SavedForm,
	type SavedQuestion,
	type DemographicForm,
	type DemographicQuestion,
	type QuestionKind,
	type Tag
} from './demographicModel';

export type ApiClient = ReturnType<typeof createApiClient>;

type ApiShape = { question_type: QuestionType2; location_format: LocationFormat | null };

const API_SHAPE: Record<QuestionKind, ApiShape> = {
	single_choice: { question_type: 'single_select', location_format: null },
	multiple_choice: { question_type: 'multi_select', location_format: null },
	number: { question_type: 'number', location_format: null },
	date: { question_type: 'date', location_format: null },
	open_text: { question_type: 'any_string', location_format: null },
	postcode: { question_type: 'location', location_format: 'postcode' },
	lat_long: { question_type: 'location', location_format: 'lat_long' },
	country: { question_type: 'location', location_format: 'country' },
	custom: { question_type: 'custom', location_format: null }
};

function kindFromVersion(version: QuestionVersionDto): QuestionKind {
	switch (version.question_type) {
		case 'single_select':
			return 'single_choice';
		case 'multi_select':
			return 'multiple_choice';
		case 'number':
			return 'number';
		case 'date':
			return 'date';
		case 'any_string':
			return 'open_text';
		case 'location':
			return version.location_format ?? 'postcode';
		case 'custom':
			return 'custom';
	}
}

function sortedLabels(options: QuestionVersionDto['options']): string[] {
	return [...options].sort((a, b) => a.position - b.position).map((option) => option.label);
}

export function questionFromDto(dto: QuestionDto): SavedQuestion {
	const content = dto.draft ?? dto.latest_published ?? null;
	const kind = content ? kindFromVersion(content) : 'single_choice';
	const choice = isChoiceKind(kind);
	return {
		id: dto.id,
		isDefault: dto.organization_id == null,
		text: content?.text ?? '',
		kind,
		datePrecision: content?.date_precision ?? 'day',
		locationCountry: content?.location_country ?? null,
		tags: dto.tags.map(({ id, name }) => ({ id, name })),
		options: content && choice ? sortedLabels(content.options) : [],
		allowOther: choice && (content?.allow_other ?? false),
		preferNotToSay: choice && (content?.prefer_not_to_say ?? false),
		maxSelections: content?.max_selections ?? null,
		specialCategory: content?.special_category ?? false,
		consentText: content?.consent_text ?? '',
		archived: Boolean(dto.archived_at),
		publishedVersion: dto.latest_published?.version_number ?? null,
		draftVersion: dto.draft?.version_number ?? null
	};
}

export function formFromDto(dto: FormDto): SavedForm {
	const content = dto.draft ?? dto.latest_published ?? null;
	const entries = content ? [...content.entries].sort((a, b) => a.position - b.position) : [];
	return {
		id: dto.id,
		name: content?.name ?? '',
		questions: entries.map((entry) => ({
			questionId: entry.question_id,
			required: entry.required,
			text: entry.question_text
		})),
		archived: Boolean(dto.archived_at),
		publishedVersion: dto.latest_published?.version_number ?? null,
		draftVersion: dto.draft?.version_number ?? null
	};
}

export function questionDraftInput(question: DemographicQuestion): QuestionDraftInput {
	const shape = API_SHAPE[question.kind];
	const choice = isChoiceKind(question.kind);
	return {
		text: question.text.trim(),
		question_type: shape.question_type,
		location_format: shape.location_format,
		location_country: shape.question_type === 'location' ? question.locationCountry : null,
		date_precision: question.kind === 'date' ? question.datePrecision : null,
		options: choice ? question.options.map((o) => o.trim()).filter((o) => o !== '') : [],
		allow_other: choice && question.allowOther,
		prefer_not_to_say: choice && question.preferNotToSay,
		max_selections: question.kind === 'multiple_choice' ? question.maxSelections : null,
		json_schema: null,
		special_category: question.specialCategory,
		consent_text: question.specialCategory ? question.consentText.trim() : null,
		tag_ids: question.tags.map((tag) => tag.id)
	};
}

export function formDraftInput(form: DemographicForm): FormDraftInput {
	return {
		name: form.name.trim(),
		entries: form.questions.map((entry, position) => ({
			position,
			question_id: entry.questionId,
			required: entry.required
		}))
	};
}

export async function loadDemographics(api: ApiClient, organizationId: string) {
	const queries = { organization: organizationId };
	const [questions, forms, tags] = await Promise.all([
		api.ListDemographicQuestions({ queries }),
		api.ListDemographicForms({ queries }),
		api.ListDemographicTags({ queries })
	]);
	return {
		questions: questions.map(questionFromDto).filter((q) => !q.archived),
		forms: forms.map(formFromDto).filter((f) => !f.archived),
		tags: tags.map((tag): Tag => ({ id: tag.id, name: tag.name }))
	};
}

/** Saves the question's draft, starting a new draft from the latest published version when needed. Resolves to the question id. */
export function persistQuestion(
	api: ApiClient,
	organizationId: string,
	question: DemographicQuestion,
	publish: boolean
) {
	return tryCatchAsync<string, ErrorType>(async () => {
		if (question.isDefault) {
			throw new Error('Duplicate the default question before editing it.');
		}
		const body = questionDraftInput(question);
		const queries = { organization: organizationId };
		let questionId = question.id;
		if (questionId === null) {
			questionId = (await api.CreateDemographicQuestion(body, { queries })).id;
		} else {
			if (question.draftVersion === null) {
				await api.StartDemographicQuestionDraft(undefined, {
					params: { question_id: questionId },
					queries
				});
			}
			await api.UpdateDemographicQuestionDraft(body, {
				params: { question_id: questionId },
				queries
			});
		}
		if (publish) {
			await api.PublishDemographicQuestion(undefined, {
				params: { question_id: questionId },
				queries
			});
		}
		return questionId;
	});
}

export function persistForm(
	api: ApiClient,
	organizationId: string,
	form: DemographicForm,
	publish: boolean
) {
	return tryCatchAsync<string, ErrorType>(async () => {
		const body = formDraftInput(form);
		const queries = { organization: organizationId };
		let formId = form.id;
		if (formId === null) {
			formId = (await api.CreateDemographicForm(body, { queries })).id;
		} else {
			if (form.draftVersion === null) {
				await api.StartDemographicFormDraft(undefined, {
					params: { form_id: formId },
					queries
				});
			}
			await api.UpdateDemographicFormDraft(body, { params: { form_id: formId }, queries });
		}
		if (publish) {
			await api.PublishDemographicForm(undefined, { params: { form_id: formId }, queries });
		}
		return formId;
	});
}

export function archiveForm(api: ApiClient, organizationId: string, formId: string) {
	return tryCatchAsync<void, ErrorType>(async () => {
		await api.ArchiveDemographicForm(undefined, {
			params: { form_id: formId },
			queries: { organization: organizationId }
		});
	});
}

export function archiveQuestion(api: ApiClient, organizationId: string, questionId: string) {
	return tryCatchAsync<void, ErrorType>(async () => {
		await api.ArchiveDemographicQuestion(undefined, {
			params: { question_id: questionId },
			queries: { organization: organizationId }
		});
	});
}

export function createDemographicTag(api: ApiClient, organizationId: string, name: string) {
	return tryCatchAsync<Tag, ErrorType>(async () => {
		const tag: TagDto = await api.CreateDemographicTag(
			{ name },
			{ queries: { organization: organizationId } }
		);
		return { id: tag.id, name: tag.name };
	});
}
