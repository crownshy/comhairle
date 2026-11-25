import { afterEach, describe, expect, it, vi } from 'vitest';
import { readStorage, writeStorage } from './browserStorage';

function stubStorage(storage: Partial<Storage>) {
	vi.stubGlobal('window', {});
	vi.stubGlobal('localStorage', storage);
}

describe('browserStorage', () => {
	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it('reads and writes through localStorage', () => {
		const values = new Map<string, string>();
		stubStorage({
			getItem: (key) => values.get(key) ?? null,
			setItem: (key, value) => void values.set(key, value)
		});

		writeStorage('key', 'value');

		expect(readStorage('key')).toBe('value');
	});

	it('reads null and skips the write when storage is blocked', () => {
		const blocked = () => {
			throw new DOMException('blocked', 'SecurityError');
		};
		stubStorage({ getItem: blocked, setItem: blocked });

		expect(readStorage('key')).toBeNull();
		expect(() => writeStorage('key', 'value')).not.toThrow();
	});
});
