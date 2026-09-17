import type { Locator } from '@playwright/test';
import { Second } from '../../src/lib/utils/units';
import { Page } from './types';

export const exists = async (locator: Locator): Promise<boolean> => !!(await locator.count());

export const generateValue = () => crypto.randomUUID().replace(/-/g, '');

export const sleep = async (seconds: number): Promise<true> =>
	new Promise((res) => {
		setTimeout(() => {
			res(true);
		}, seconds * Second);
	});

export async function testWithRefresh(page: Page, tests: () => Promise<void>) {
	await tests();
	await page.reload();
	await tests();
}
