import type { QuestionDto } from '@crownshy/api-client/api';

function getVersion(question: QuestionDto): QuestionDto['versions'][number] | undefined {
	const versionNumber = question.latest_published?.version_number;
	if (!versionNumber) {
		return;
	}

	const version = question.versions.find((v) => v.version_number === versionNumber);

	return version;
}

export default {
	getVersion
};
