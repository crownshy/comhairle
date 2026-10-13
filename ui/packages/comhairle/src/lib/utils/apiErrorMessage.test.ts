import { describe, it, expect } from 'vitest';
import { apiErrorMessage } from './apiErrorMessage';

describe('apiErrorMessage', () => {
	it('returns the message from the API error body', () => {
		const error = {
			response: { data: { err: 'Step uses a question the organisation cannot see' } }
		};
		expect(apiErrorMessage(error, 'fallback')).toBe(
			'Step uses a question the organisation cannot see'
		);
	});

	it('falls back when the error has no response body', () => {
		expect(apiErrorMessage(new Error('boom'), 'fallback')).toBe('fallback');
		expect(apiErrorMessage({ response: {} }, 'fallback')).toBe('fallback');
		expect(apiErrorMessage({ response: { data: { err: '' } } }, 'fallback')).toBe('fallback');
		expect(apiErrorMessage(null, 'fallback')).toBe('fallback');
	});
});
