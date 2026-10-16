import { expect, Locator } from '@playwright/test';
import { Second } from '../../src/lib/utils/units';
import { Page } from './types';
import { eventually } from './testing';

export const exists = async (locator: Locator): Promise<boolean> => !!(await locator.count());

export const generateValue = () => crypto.randomUUID().replace(/-/g, '');

export const sleep = async (seconds: number): Promise<true> =>
	new Promise((res) => {
		setTimeout(() => {
			res(true);
		}, seconds * Second);
	});

/**
 * @description Run tests, then refresh the page and runs the same tests again. Used to make sure that nothing changes with the refresh
 */
export async function testWithRefresh(page: Page, tests: () => Promise<void>) {
	await tests();
	await sleep(1.5);
	await page.reload();
	await sleep(1.5);
	await tests();
}

export async function find(locator: Locator) {
	await eventually(() => expect(locator).toBeVisible());
	return locator;
}
