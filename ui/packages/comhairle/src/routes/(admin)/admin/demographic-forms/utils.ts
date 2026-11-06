import type { DemographicFormLatestDto, QuestionLatestDto } from '@crownshy/api-client/api';

export const specialCategoryCount = (
	form: DemographicFormLatestDto,
	questions: QuestionLatestDto[]
) =>
	form?.entries.filter((fq) => questions?.find((q) => q.id === fq.questionId)?.specialCategory)
		.length;
