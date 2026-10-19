import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';
import { Button } from './utils/components';
import { testWithRefresh } from './utils';
import { expect, Locator } from '@playwright/test';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Facilitators');
});

const isSelected = (locator: Locator) =>
	locator.getAttribute('aria-pressed').then((state) => state === 'true');

test('Events/Facilitators page', async ({ page }) => {
	const AttendeeBtn = await Button('Attendee', page);
	const FacilitatorBtn = await Button('Facilitator', page);
	const ModeratorBtn = await Button('Moderator', page);

	if (!(await isSelected(FacilitatorBtn))) await FacilitatorBtn.click();

	await AttendeeBtn.click();
	await testWithRefresh(page, async () => {
		expect(await isSelected(AttendeeBtn)).toBe(true);
	});

	await ModeratorBtn.click();
	await testWithRefresh(page, async () => {
		expect(await isSelected(ModeratorBtn)).toBe(true);
	});

	await FacilitatorBtn.click();
	await testWithRefresh(page, async () => {
		expect(await isSelected(FacilitatorBtn)).toBe(true);
	});
});
