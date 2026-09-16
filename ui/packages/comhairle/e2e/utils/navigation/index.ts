import { Page } from '../types';

export async function gotoComhairle(page: Page) {
	await page.goto('http://localhost:4173/');
}
