import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { Switches } from './utils/components';
import { test } from './utils/testing';
import { testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Access');
});

test('Configure/Access page', async ({ page, cleanup }) => {
	const switches = await Switches(
		[
			['show_conversation_publicy', 'Show conversation publicly'],
			['only_allow_participation_by', 'Only allow participation by'],
			['automatically_log_in_with_an', 'Automatically log in with an'],
			['enable_signup_prompts', 'Enable signup prompts'],
			['show_thank_you_page_anonymous', 'Show thank you page anonymous'],
			['show_thank_you_page_feedback', 'Show thank you page feedback'],
			['allow_revisit_after_finishing', 'Allow revisit after finishing']
		],
		{ page, cleanup }
	);

	await switches.toggle('show_conversation_publicy');
	await switches.toggle('only_allow_participation_by');
	await switches.toggle('automatically_log_in_with_an');
	await switches.toggle('enable_signup_prompts');
	await switches.toggle('show_thank_you_page_anonymous');
	await switches.toggle('show_thank_you_page_feedback');
	await switches.toggle('allow_revisit_after_finishing');

	await testWithRefresh(page, async () => {
		await switches.expect();
	});

	// TODO: Add in cohost test. It might be moved in the future which is why it's currently been ignored.
});
