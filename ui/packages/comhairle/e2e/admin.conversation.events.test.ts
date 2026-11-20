import { expect, type Route } from '@playwright/test';
import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events, { TEST_EVENT_NAME } from './utils/navigation/Events';
import type { Page } from './utils/types';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
});

// The events strip above the page has its own Add event link, so the page's makes two.
const addEventLinks = (page: Page) =>
	page.getByRole('main').getByRole('link', { name: 'Add event' });

const isListEventsRequest = (url: URL) => /\/conversation\/[^/]+\/events$/.test(url.pathname);

/**
 * The events list streams in from the page load, so only a client-side visit fetches it from
 * the browser where `page.route` can see it. Leave the tab and come back to trigger one.
 */
async function reopenEventsWith(page: Page, handler: (route: Route) => Promise<void>) {
	await page.route(isListEventsRequest, (route) =>
		route.request().method() === 'GET' ? handler(route) : route.fallback()
	);
	await Conversation.openTab(page, 'Process design', undefined);
	await Conversation.openTab(page, 'Events', undefined);
}

test('has title', async ({ page }) => {
	await expect(
		page.getByRole('main').getByRole('heading', { name: 'Events', level: 1 })
	).toBeVisible();
});

test('Events page - Add event', async ({ page }) => {
	await Events.add({ page, withTests: true });
});

test('Events page - Edit event opens the event', async ({ page }) => {
	await Events.gotoPlaywrightEvent(page);

	await page.waitForURL(/\/events\/[^/]+\/?$/);
	await expect(page.getByRole('heading', { name: TEST_EVENT_NAME }).first()).toBeVisible();
});

test('Events page - shows a loading message until the events arrive', async ({ page }) => {
	let release = () => {};
	const released = new Promise<void>((resolve) => (release = resolve));

	await reopenEventsWith(page, async (route) => {
		await released;
		await route.continue();
	});

	const main = page.getByRole('main');
	await expect(main.getByText('Loading events')).toBeVisible();
	await expect(addEventLinks(page)).toHaveCount(1);

	release();

	await expect(main.getByText('Loading events')).toBeHidden();
	await expect(addEventLinks(page)).toHaveCount(2);
});

test('Events page - shows an empty state when there are no events', async ({ page }) => {
	await reopenEventsWith(page, (route) => route.fulfill({ json: { records: [], total: 0 } }));

	const main = page.getByRole('main');
	await expect(main.getByText('No events yet.')).toBeVisible();
	await expect(addEventLinks(page)).toHaveCount(2);
	await expect(main.getByRole('link', { name: 'Edit event' })).toHaveCount(0);
});

test('Events page - shows an error when the events fail to load', async ({ page }) => {
	await reopenEventsWith(page, (route) => route.fulfill({ status: 500, json: {} }));

	const main = page.getByRole('main');
	await expect(main.getByText('Could not load events, please try again')).toBeVisible();
	await expect(main.getByText('No events yet.')).toBeHidden();
	await expect(addEventLinks(page)).toHaveCount(1);
});
