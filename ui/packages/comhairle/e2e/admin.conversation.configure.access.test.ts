import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { Switch } from './utils/components';
import { test } from './utils/testing';
import { testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Access');
});

test('Configure/Access page', async ({ page, cleanup }) => {
	const ShowConversationPublicy = await Switch('Show conversation publicly', { page, cleanup });
	const OnlyAllowParticipationBy = await Switch('Only allow participation by', { page, cleanup });
	const AutomaticallyLogInWithAn = await Switch('Automatically log in with an', {
		page,
		cleanup
	});
	const EnableSignupPrompts = await Switch('Enable signup prompts', { page, cleanup });
	const ShowThankYouPageAnonymous = await Switch('Show thank you page anonymous', {
		page,
		cleanup
	});
	const ShowThankYouPageFeedback = await Switch('Show thank you page feedback', {
		page,
		cleanup
	});
	const AllowRevisitAfterFinishing = await Switch('Allow revisit after finishing', {
		page,
		cleanup
	});

	await ShowConversationPublicy.toggle();
	await OnlyAllowParticipationBy.toggle();
	await AutomaticallyLogInWithAn.toggle();
	await EnableSignupPrompts.toggle();
	await ShowThankYouPageAnonymous.toggle();
	await ShowThankYouPageFeedback.toggle();
	await AllowRevisitAfterFinishing.toggle();

	await testWithRefresh(page, async () => {
		await ShowConversationPublicy.expect();
		await OnlyAllowParticipationBy.expect();
		await AutomaticallyLogInWithAn.expect();
		await EnableSignupPrompts.expect();
		await ShowThankYouPageAnonymous.expect();
		await ShowThankYouPageFeedback.expect();
		await AllowRevisitAfterFinishing.expect();
	});

	// TODO: Add in cohost test. It might be moved in the future which is why it's currently been ignored.
});
