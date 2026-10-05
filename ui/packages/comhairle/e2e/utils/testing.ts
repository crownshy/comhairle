import { test as base, expect } from '@playwright/test';
import { gotoComhairle } from './navigation';
import { sleep } from '.';
import { Minute, Second } from '../../src/lib/utils/units';

type CleanupCallback = () => Promise<void>;

type Fixtures = {
	forEachTest: void;
	cleanup: (callback: CleanupCallback) => void;
};

export const test = base.extend<Fixtures>({
	forEachTest: [
		async ({ page }, use) => {
			await gotoComhairle(page);
			await use();
		},
		{ auto: true }
	],

	// Can't use _ have to use {} for playwright to run apparently
	// eslint-disable-next-line no-empty-pattern
	cleanup: async ({}, use) => {
		const cleanups: CleanupCallback[] = [];

		await use((fn) => cleanups.push(fn));

		for (const cleanup of cleanups) {
			await cleanup();
		}

		// Wait for fields to save before exiting
		await sleep(1.5);
	}
});

/**
 * @important *IMPORTANT: Make sure to use an `expect` inside of the function callback*
 * @description Poll periodically to check whether the test has passed. Used for things which take a variable amount of time to update, such as saving spinners or fetch requests
 * @param callback - The `expect` you want to check
 * @param timeout - Timeout (in minutes) until the test gives up and fails, defaults to 1 minute
 */
export const eventually = async (callback: () => Promise<void>, timeout?: number) => {
	await expect(callback).toPass({
		intervals: [5 * Second],
		timeout: (timeout ?? 1) * Minute
	});
};
