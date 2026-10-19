import { test, eventually } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';
import { Button, RadioButtons, Textbox } from './utils/components';
import { generateValue, sleep } from './utils';
import { expect } from '@playwright/test';

type Options = ['never', '1 Day', '1 Week', '1 Month', 'custom'];

const generateEmail = () => generateValue().slice(10) + '@gmail.com';

test.describe('Events/Invites page', () => {
	let Email: Awaited<ReturnType<typeof Textbox>> | undefined;
	let ValidFor: Awaited<ReturnType<typeof RadioButtons<Options>>> | undefined;
	let Submit: Awaited<ReturnType<typeof Button>> | undefined;

	test.beforeEach(async ({ page }) => {
		await login(page);
		await Conversation.open(page);
		await Conversation.openTab(page, 'Events', undefined);
		await Events.gotoPlaywrightEvent(page);
		await Events.subtab(page, 'Invites');
		await sleep(1);
		Email = await Textbox('Emails', { page });
		ValidFor = await RadioButtons(['never', '1 Day', '1 Week', '1 Month', 'custom'], {
			page
		});
		Submit = await Button('Submit', page);
	});

	test('Should show an error for incorrect email addresses', async () => {
		await Email?.write('incorrect_email');
		expect(await Email?.isValid()).toBe(false);
	});

	test('Should be able to add a new email', async ({ page }) => {
		await Email?.write(generateEmail());
		await ValidFor?.select('1 Day');
		await Submit?.click();
		await eventually(() => expect(page.getByText(Email?.value() ?? '')).toBeVisible());
	});

	test('Should be able to add multiple new emails', async ({ page }) => {
		await Email?.write(`${generateEmail()},${generateEmail()}`);
		await ValidFor?.select('1 Day');
		await Submit?.click();
		Email?.value()
			.split(',')
			.forEach((value) =>
				eventually(() => expect(page.getByText(value.trim() ?? '')).toBeVisible())
			);
	});
});
