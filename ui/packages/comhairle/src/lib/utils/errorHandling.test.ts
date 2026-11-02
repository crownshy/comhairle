import { describe, expect, it } from 'vitest';
import { apiErrorStatus } from './errorHandling';

describe('apiErrorStatus', () => {
	it('reads the status from an API client error', () => {
		expect(apiErrorStatus({ response: { status: 401, data: { err: 'x' } } })).toBe(401);
	});

	it('is undefined for errors without a response', () => {
		expect(apiErrorStatus(new Error('offline'))).toBeUndefined();
		expect(apiErrorStatus(null)).toBeUndefined();
		expect(apiErrorStatus({ response: {} })).toBeUndefined();
	});
});
