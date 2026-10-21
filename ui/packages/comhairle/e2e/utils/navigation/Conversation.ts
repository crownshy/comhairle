import { expect } from '@playwright/test';
import { TEST_CONVERSATION_TITLE, TEST_LAUNCHED_CONVERSATION_TITLE } from '../constants';
import type { Page } from '../types';

type Tabs = {
	Configure: readonly ['Details', 'Content', 'Glossary', 'Moderation policy', 'Access', 'Team'];
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
	await page.goto('/admin/conversations');
	const list = page.getByRole('main');
	await expect(list.getByRole('heading', { name: 'Your conversations', level: 1 })).toBeVisible();

	// The innermost element holding both the card heading and its edit link is the card.
	const card = list
		.locator('*')
		.filter({ has: page.getByRole('heading', { name: title, exact: true, level: 2 }) })
		.filter({ has: page.getByRole('link', { name: 'Edit conversation' }) })
		.last();

	if ((await card.count()) > 0) {
		await card.getByRole('link', { name: 'Edit conversation' }).click();
	} else {
		await list.getByRole('button', { name: 'Create new conversation' }).click();
		await page.getByRole('menuitem', { name: 'Start from blank' }).click();
		await page.getByRole('textbox', { name: 'Title', exact: true }).fill(title);
	}

	await expect(page.getByRole('heading', { name: title, exact: true, level: 1 })).toBeVisible();
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
		await page.getByRole('button', { name: 'Launch conversation' }).click();
		await page.getByRole('dialog').getByRole('button', { name: 'Launch', exact: true }).click();
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
