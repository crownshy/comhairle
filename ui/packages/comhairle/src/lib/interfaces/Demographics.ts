import { typed } from '$lib/utils/types';
import type {
	DemographicForm,
	DemographicFormEntryDto,
	DemographicFormLatestDto,
	QuestionLatestDto,
	QuestionRow,
	QuestionType,
	VersionStatus
} from '@crownshy/api-client/api';

const newForm = (organisationId: string): DemographicFormLatestDto => ({
	entries: typed<DemographicFormEntryDto[]>([]),
	form_id: 'f-0',
	id: '0',
	meta: typed<DemographicForm>({
		id: '',
		organization_id: organisationId,
		created_at: ''
	}),
	name: '',
	status: typed<VersionStatus>('draft'),
	version_number: 1,
	draftVersion: 0,
	publishedVersion: 0,
	updated_at: ''
});

const newQuestion = (organisationId: string): QuestionLatestDto => ({
	allowOther: false,
	id: crypto.randomUUID(),
	meta: typed<QuestionRow>({
		id: '0',
		createdAt: '',
		organizationId: organisationId
	}),
	multi: false,
	options: [],
	preferNotToSay: false,
	publishedVersion: 0,
	questionId: crypto.randomUUID(),
	questionType: typed<QuestionType>('text'),
	specialCategory: false,
	status: typed<VersionStatus>('draft'),
	tags: [],
	text: '',
	updatedAt: '',
	draftVersion: 0,
	versionNumber: 0
});

function isDefaultQuestion(question: QuestionLatestDto): boolean {
	return question.meta.organizationId === null;
}

export default {
	newForm,
	newQuestion,
	isDefaultQuestion
};
