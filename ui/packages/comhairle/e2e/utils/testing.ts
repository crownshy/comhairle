import { test as base } from '@playwright/test';
import { gotoComhairle } from './navigation';
import { sleep } from '.';

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

		await Promise.all(cleanups.map((fn) => fn()));

		// Wait for fields to save before exiting
		await sleep(1.5);
	}
});
