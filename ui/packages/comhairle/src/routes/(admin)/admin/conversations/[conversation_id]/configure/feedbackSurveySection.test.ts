import { describe, it, expect } from 'vitest';
import { render } from 'svelte/server';
import FeedbackSurveySection from './FeedbackSurveySection.svelte';
import type { ConversationWithTranslations, FeedbackSurveyDto } from '@crownshy/api-client/api';

const conversation = { id: 'c1', ownerId: 'u1' } as unknown as ConversationWithTranslations;

const survey: FeedbackSurveyDto = {
	conversationId: 'c1',
	completed: false,
	toolConfig: {
		type: 'heyform',
		survey_id: 'form-1',
		survey_url: 'forms.example.com/form/form-1',
		server_url: 'forms.example.com',
		admin_user: 'admin@example.com',
		admin_password: 'secret',
		workspace_id: 'ws',
		project_id: 'proj'
	}
};

describe('FeedbackSurveySection', () => {
	it('offers to create the survey when the conversation has none', () => {
		const { body } = render(FeedbackSurveySection, {
			props: { conversation, workflowId: 'w1', survey: null, insights: null }
		});

		expect(body).toContain('Create feedback survey');
		expect(body).not.toContain('Remove feedback survey');
		expect(body).not.toContain('<iframe');
	});

	it('shows the builder, the responses and removal once it exists', () => {
		const { body } = render(FeedbackSurveySection, {
			props: { conversation, workflowId: 'w1', survey, insights: [] }
		});

		expect(body).toContain('<iframe');
		expect(body).toContain('No responses yet');
		expect(body).toContain('Remove feedback survey');
		expect(body).toContain('/conversations/c1/preview/workflow/w1/thank_you');
		expect(body).not.toContain('Create feedback survey');
	});

	it('has no preview link without a workflow to preview', () => {
		const { body } = render(FeedbackSurveySection, {
			props: { conversation, workflowId: undefined, survey, insights: [] }
		});

		expect(body).not.toContain('Preview the thank-you page');
	});
});
