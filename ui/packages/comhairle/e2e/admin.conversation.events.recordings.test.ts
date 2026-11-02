import { test } from './utils/testing';
import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import Events from './utils/navigation/Events';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Events', undefined);
	await Events.gotoPlaywrightEvent(page);
	await Events.subtab(page, 'Recordings');
});

// TODO: Add these at some point, current it's erroring and don't know how easy it is to make it upload files either
test('Events/Recordings page', async ({ page }) => {});
