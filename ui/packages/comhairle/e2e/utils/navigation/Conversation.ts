import { expect } from '@playwright/test';
import { TEST_CONVERSATION_TITLE, TEST_LAUNCHED_CONVERSATION_TITLE } from '../constants';
import type { Page } from '../types';

type Tabs = {
	Configure: readonly ['Details', 'Content', 'Moderation policy', 'Team'];
	'Process design': undefined;
	'Learning Assistant': undefined;
	Events: undefined;
	Recruit: readonly ['Email', 'Open Links'];
	Monitor: undefined;
	Notify: undefined;
	Report: undefined;
};

type Subtabs<T extends keyof Tabs> = Tabs[T] extends readonly string[]
	? Tabs[T][number] | undefined
	: undefined;

// Engage and Results fold in the sidebar, so their links only exist once the group is open.
const FOLDED_GROUP: Partial<Record<keyof Tabs, string>> = {
	Recruit: 'Engage',
	Monitor: 'Engage',
	Notify: 'Engage',
	Report: 'Results'
};

const sidebar = (page: Page) => page.getByRole('list', { name: 'Conversation sections' });

// "Process design" carries a step count in its accessible name ("Process design 2 steps").
const sidebarLink = (page: Page, tab: keyof Tabs) =>
	sidebar(page).getByRole('link', {
		name: tab === 'Process design' ? /^Process design/ : tab,
		exact: tab !== 'Process design'
	});

async function openOrCreate(page: Page, title: string) {
	// Search rather than scan, because the list is paginated.
	await page.goto(`/admin/conversations?q=${encodeURIComponent(title)}`);
	const list = page.getByRole('main');
	await expect(list.getByRole('heading', { name: 'Your conversations', level: 1 })).toBeVisible();

	const card = list
		.getByRole('article')
		.filter({ has: page.getByRole('link', { name: title, exact: true }) });

	if ((await card.count()) > 0) {
		await card.first().getByRole('link', { name: 'Edit conversation' }).click();
		await page.waitForURL(/\/admin\/conversations\/[^/?]+\//);
	} else {
		// The list is server-rendered, so the button shows before the app has hydrated and an
		// early click does nothing. Retry until the menu opens.
		const startFromBlank = page.getByRole('menuitem', { name: 'Start from blank' });
		await expect(async () => {
			await list.getByRole('button', { name: 'Create new conversation' }).click();
			await expect(startFromBlank).toBeVisible({ timeout: 1000 });
		}).toPass();
		await startFromBlank.click();
		// Same race on the new conversation's Details page: typing before hydration never saves.
		await page.waitForURL(/\/configure\/details$/);
		await page.waitForLoadState('networkidle');
		// The title saves after a debounce, and reloading before then would cancel it.
		const saved = page.waitForResponse(
			(response) => response.url().includes('/api/') && response.request().method() !== 'GET'
		);
		await page.getByRole('textbox', { name: 'Title', exact: true }).fill(title);
		await saved;
	}

	// Autosave doesn't reload the page, so a just-renamed conversation only shows its title
	// in the sidebar after a reload. Reload until the save has landed.
	const heading = page.getByRole('heading', { name: title, exact: true, level: 1 });
	await expect(async () => {
		if (!(await heading.isVisible())) await page.reload();
		await expect(heading).toBeVisible({ timeout: 2000 });
	}).toPass();
}

const Conversation = {
	open: (page: Page) => openOrCreate(page, TEST_CONVERSATION_TITLE),
	/**
	 * Recruit, Notify, Monitor and Report only open once a conversation is live, and launching
	 * locks the set-up, so these tests use their own conversation rather than the shared one.
	 */
	openLaunched: async (page: Page) => {
		await openOrCreate(page, TEST_LAUNCHED_CONVERSATION_TITLE);

		// Screen readers hear "Launched , done" once the stage is complete.
		const launched = sidebar(page).getByRole('link', { name: /^Launched\b/ });
		if ((await launched.count()) > 0) {
			return;
		}

		await sidebar(page).getByRole('link', { name: 'Launch', exact: true }).click();
		const confirm = page
			.getByRole('dialog')
			.getByRole('button', { name: 'Launch', exact: true });
		// Retry like the create button: a click before hydration doesn't open the dialog.
		await expect(async () => {
			await page.getByRole('button', { name: 'Launch conversation' }).click();
			await expect(confirm).toBeVisible({ timeout: 1000 });
		}).toPass();
		await confirm.click();
		await expect(launched).toBeVisible();
	},
	openTab: async <T extends keyof Tabs>(page: Page, tab: T, subtab: Subtabs<T>) => {
		const link = sidebarLink(page, tab);
		const group = FOLDED_GROUP[tab];
		if (group && !(await link.isVisible())) {
			await sidebar(page).getByRole('button', { name: group }).click();
		}

		await link.click();
		await expect(link).toHaveAttribute('aria-current', 'page');

		if (subtab === undefined) {
			return;
		}

		// Each sub-tab marks itself active only once its page has loaded.
		if (tab === 'Configure') {
			const configureTab = page
				.getByRole('main')
				.getByRole('tab', { name: subtab, exact: true });
			await configureTab.click();
			await expect(configureTab).toHaveAttribute('aria-selected', 'true');
			return;
		}
		const subtabLink = page
			.getByRole('navigation', { name: 'Sub sections' })
			.getByRole('link', { name: subtab, exact: true });
		await subtabLink.click();
		await expect(subtabLink).toHaveAttribute('aria-current', 'page');
	}
};

export default Conversation;
