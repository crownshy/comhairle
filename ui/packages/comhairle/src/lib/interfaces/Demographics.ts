import type { QuestionDto, QuestionType2 } from '@crownshy/api-client/api';

function getVersion(question: QuestionDto): QuestionDto['versions'][number] | undefined {
	const versionNumber = question.latest_published?.version_number;
	if (!versionNumber) {
		return;
	}

	const version = question.versions.find((v) => v.version_number === versionNumber);

	return version;
}

function getQuestionFromType(questionType: QuestionType2): string {
	switch (questionType) {
		case 'number':
			return 'Number';
		case 'single_select':
			return 'Single choice';
		case 'multi_select':
			return 'Multiple choice';
		case 'date':
			return 'Date';
		case 'any_string':
			return 'Text';
		case 'location':
			return 'Location/Postcode';
		case 'custom':
		default:
			return 'Custom';
	}
}

export default {
	getVersion,
	getQuestionFromType
};
