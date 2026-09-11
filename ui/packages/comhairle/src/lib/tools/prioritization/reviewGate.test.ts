import { describe, it, expect } from 'vitest';
import { requiredReviewCount, canLeaveStep } from './reviewGate';

describe('requiredReviewCount', () => {
	it('requires every proposal when the admin has set no minimum', () => {
		expect(requiredReviewCount(null, 9)).toBe(9);
		expect(requiredReviewCount(undefined, 9)).toBe(9);
	});

	it('uses the configured minimum when it loosens the default', () => {
		expect(requiredReviewCount(3, 9)).toBe(3);
	});

	it('clamps a minimum set above the number of proposals', () => {
		expect(requiredReviewCount(20, 9)).toBe(9);
	});

	it('treats a non-positive minimum as one proposal', () => {
		expect(requiredReviewCount(0, 9)).toBe(1);
		expect(requiredReviewCount(-4, 9)).toBe(1);
	});

	it('requires nothing when the step has no proposals', () => {
		expect(requiredReviewCount(null, 0)).toBe(0);
		expect(requiredReviewCount(3, 0)).toBe(0);
	});
});

describe('canLeaveStep', () => {
	it('stays closed while proposals are still loading', () => {
		expect(canLeaveStep({ reviewedCount: 0, requiredReviews: 0, proposalsLoaded: false })).toBe(
			false
		);
	});

	it('stays closed when the participant is short of the minimum', () => {
		expect(canLeaveStep({ reviewedCount: 2, requiredReviews: 3, proposalsLoaded: true })).toBe(
			false
		);
	});

	it('opens once the minimum is met', () => {
		expect(canLeaveStep({ reviewedCount: 3, requiredReviews: 3, proposalsLoaded: true })).toBe(
			true
		);
	});

	it('opens when the participant has reviewed more than the minimum', () => {
		expect(canLeaveStep({ reviewedCount: 9, requiredReviews: 3, proposalsLoaded: true })).toBe(
			true
		);
	});

	it('opens on a loaded step that has no proposals to review', () => {
		expect(canLeaveStep({ reviewedCount: 0, requiredReviews: 0, proposalsLoaded: true })).toBe(
			true
		);
	});
});
