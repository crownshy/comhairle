import { TEST_CONVERSATION_TITLE } from '../constants';
import { exists } from '..';
import type { Page } from '../types';

type Tabs = {
	Configure: readonly ['Details', 'Content', 'Glossary', 'Access', 'Team'];
	'Process Design': readonly ['Add step'];
	'Learning Assistant': undefined;
	Events: readonly ['Add event'];
	Recruit: readonly ['Email', 'Open Links'];
	Monitor: undefined;
	Notify: undefined;
	Report: undefined;
};

type Subtabs<T extends keyof Tabs> = Tabs[T] extends readonly string[]
	? Tabs[T][number] | undefined
	: undefined;

const Conversation = {
	open: async (page: Page) => {
		const conversation = page.getByRole('link', { name: TEST_CONVERSATION_TITLE });

		// Conversation already exists
		if (await exists(conversation)) {
			conversation.click();
			return;
		}

		// Make conversation
		await page.getByText('Start from blank').click();
		await page.getByRole('menuitem', { name: 'Start from blank' }).click();
		await page.getByRole('textbox', { name: 'Title' }).fill(TEST_CONVERSATION_TITLE);
	},
	/**
	 * Pre-requisite is that it should be on an admin dashboard conversation
	 */
	launch: async (page: Page) => {
		const launched = page.getByText('Launched');
		if (await exists(launched)) {
			return;
		}

		const launchConversation = page.getByRole('button', { name: 'Launch Conversation' });
		if (launchConversation) {
			await launchConversation.click();
			await page.getByRole('button', { name: 'Launch', exact: true }).click();
			await page.getByRole('link', { name: 'Live Conversation Link' }).click();
		}

		throw new Error(
			"Could not find 'Launched' text, or 'Launch' button, is this being called on a conversation page?"
		);
	},
	openTab: async <T extends keyof Tabs>(page: Page, tab: T, subtab: Subtabs<T>) => {
		await page.getByRole('link', { name: tab }).click();

		if (subtab === undefined) {
			return;
		}

		switch (tab) {
			case 'Configure':
			case 'Recruit':
				await page.getByRole('link', { name: subtab as string }).click();
				return;
			case 'Process Design':
				// TODO:Add if statements here
				await page.getByRole('button', { name: 'Add step' }).click();
				return;
			case 'Events':
				await page.getByRole('link', { name: 'Add event' }).click();
				return;
			case 'Learning Assistant':
			case 'Monitor':
			case 'Notify':
			case 'Report':
		}
	}
};

export default Conversation;
