import { describe, expect, it, vi } from 'vitest';
import type { QuestionDto } from '@crownshy/api-client/api';
import { createApiClient } from '@crownshy/api-client/client';
import { persistQuestion, questionFromDto } from './demographicApi';
import { blankQuestion, duplicateQuestionDraft, nextVersionNumber } from './demographicModel';

const organizationId = '00000000-0000-0000-0000-000000000001';
const questionId = '00000000-0000-0000-0000-000000000002';
const copyId = '00000000-0000-0000-0000-000000000003';

function questionDto(owner: string | null): QuestionDto {
	return {
		id: questionId,
		organization_id: owner,
		tags: [],
		versions: [],
		latest_published: {
			id: '00000000-0000-0000-0000-000000000004',
			question_id: questionId,
			text: 'What is your age?',
			question_type: 'number',
			allow_other: false,
			prefer_not_to_say: false,
			special_category: false,
			options: [],
			status: 'published',
			version_number: 3
		}
	};
}

describe('default demographic questions', () => {
	it('identifies defaults from ownership, not question text or version', () => {
		expect(questionFromDto(questionDto(null)).isDefault).toBe(true);
		expect(questionFromDto(questionDto(organizationId)).isDefault).toBe(false);
		expect(blankQuestion().isDefault).toBe(false);
	});

	it('duplicates content without identity, ownership or version state', () => {
		const source = {
			...questionFromDto(questionDto(null)),
			tags: [{ id: 'age', name: 'Age' }],
			options: ['Under 18', '18 or over'],
			draftVersion: 4
		};
		const duplicate = duplicateQuestionDraft(source);

		expect(duplicate).toEqual({
			...source,
			id: null,
			isDefault: false,
			text: 'What is your age? (copy)',
			publishedVersion: null,
			draftVersion: null
		});
		expect(nextVersionNumber(duplicate)).toBe(1);
		duplicate.options[0] = 'Under 21';
		duplicate.tags[0].name = 'Changed';
		expect(source.options[0]).toBe('Under 18');
		expect(source.tags[0].name).toBe('Age');
	});

	it('rejects saving a default without making any API requests', async () => {
		const api = createApiClient('/api', undefined, 'client');
		const request = vi.spyOn(api, 'request');
		const result = await persistQuestion(
			api,
			organizationId,
			questionFromDto(questionDto(null)),
			true
		);

		expect(result.err).toEqual(new Error('Duplicate the default question before editing it.'));
		expect(request).not.toHaveBeenCalled();
	});

	it('creates and publishes a duplicate under its new id', async () => {
		const api = createApiClient('/api', undefined, 'client');
		const response = { ...questionDto(organizationId), id: copyId };
		const create = vi.spyOn(api, 'CreateDemographicQuestion').mockResolvedValue(response);
		const publish = vi.spyOn(api, 'PublishDemographicQuestion').mockResolvedValue(response);
		const start = vi.spyOn(api, 'StartDemographicQuestionDraft');
		const update = vi.spyOn(api, 'UpdateDemographicQuestionDraft');
		const duplicate = duplicateQuestionDraft(questionFromDto(questionDto(null)));
		const result = await persistQuestion(api, organizationId, duplicate, true);

		expect(result).toEqual({ ok: copyId, err: null });
		expect(create).toHaveBeenCalledWith(
			expect.objectContaining({ text: 'What is your age? (copy)' }),
			{ queries: { organization: organizationId } }
		);
		expect(publish).toHaveBeenCalledWith(undefined, {
			params: { question_id: copyId },
			queries: { organization: organizationId }
		});
		expect(start).not.toHaveBeenCalled();
		expect(update).not.toHaveBeenCalled();
	});

	it('still starts, updates and publishes drafts for organisation-owned questions', async () => {
		const api = createApiClient('/api', undefined, 'client');
		const response = questionDto(organizationId);
		const start = vi.spyOn(api, 'StartDemographicQuestionDraft').mockResolvedValue(response);
		const update = vi.spyOn(api, 'UpdateDemographicQuestionDraft').mockResolvedValue(response);
		const publish = vi.spyOn(api, 'PublishDemographicQuestion').mockResolvedValue(response);
		const create = vi.spyOn(api, 'CreateDemographicQuestion');
		const result = await persistQuestion(api, organizationId, questionFromDto(response), true);

		expect(result).toEqual({ ok: questionId, err: null });
		expect(start).toHaveBeenCalledOnce();
		expect(update).toHaveBeenCalledOnce();
		expect(publish).toHaveBeenCalledOnce();
		expect(create).not.toHaveBeenCalled();
	});
});
