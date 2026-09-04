import { afterEach, describe, expect, it, vi } from 'vitest';
import { haptic } from './haptics';

describe('haptic', () => {
	const original = Object.getOwnPropertyDescriptor(navigator, 'vibrate');

	afterEach(() => {
		if (original) Object.defineProperty(navigator, 'vibrate', original);
		else Reflect.deleteProperty(navigator, 'vibrate');
	});

	it('is a no-op where the Vibration API is missing', () => {
		Reflect.deleteProperty(navigator, 'vibrate');
		expect(haptic('light')).toBe(false);
	});

	it('passes the pattern for the kind through to navigator.vibrate', () => {
		const vibrate = vi.fn(() => true);
		Object.defineProperty(navigator, 'vibrate', { value: vibrate, configurable: true });
		expect(haptic('success')).toBe(true);
		expect(vibrate).toHaveBeenCalledWith([18, 70, 36]);
	});

	it('swallows a browser that throws', () => {
		Object.defineProperty(navigator, 'vibrate', {
			value: () => {
				throw new Error('nope');
			},
			configurable: true
		});
		expect(haptic('medium')).toBe(false);
	});
});
