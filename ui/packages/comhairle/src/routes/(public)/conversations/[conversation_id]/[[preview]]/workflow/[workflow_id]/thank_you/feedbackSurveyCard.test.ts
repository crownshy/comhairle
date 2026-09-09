import { describe, it, expect } from 'vitest';
import { render } from 'svelte/server';
import FeedbackSurveyCard from './FeedbackSurveyCard.svelte';

const survey = {
	conversationId: 'c1',
	userId: 'u1',
	surveyId: 'form-1',
	serverUrl: 'forms.example.com',
	surveyUrl: 'forms.example.com/form/form-1'
};

describe('FeedbackSurveyCard', () => {
	it('opens on the invitation, with the survey behind the button rather than on screen', () => {
		const { body } = render(FeedbackSurveyCard, { props: { ...survey, completed: false } });

		expect(body).toContain('best person to tell us how it went');
		expect(body).toContain('Tell us how it went');
		// The form is only fetched once asked for: no overlay, no iframe, until the button is pressed.
		expect(body).not.toContain('role="dialog"');
		expect(body).not.toContain('<iframe');
		expect(body).not.toContain('Thanks, that helps');
	});

	it('thanks a participant the API says has already finished, on any device', () => {
		const { body } = render(FeedbackSurveyCard, { props: { ...survey, completed: true } });

		expect(body).toContain('Thanks, that helps');
		expect(body).not.toContain('Tell us how it went');
	});
});
