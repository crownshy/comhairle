import { test } from './utils/testing';
import Conversation from './utils/navigation/Conversation';
import { Textbox, CollapsibleRichField } from './utils/components';
import { login } from './utils/auth';
import { testWithRefresh } from './utils';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Content');
});

test('Configure/Content page', async ({ page, cleanup }) => {
	const PrivacyPolicy = await CollapsibleRichField('Add privacy policy', 0, { page, cleanup });
	const ShortPrivacyPolicy = await CollapsibleRichField('Add short privacy policy', 1, {
		page,
		cleanup
	});
	const Faqs = await CollapsibleRichField('Add faqs', 2, { page, cleanup });
	const ThankYou = await CollapsibleRichField('Add thank you message', 3, { page, cleanup });
	const CTA = await Textbox('Call to action', { page, cleanup });

	await PrivacyPolicy.write();
	await ShortPrivacyPolicy.write();
	await Faqs.write();
	await ThankYou.write();
	await CTA.write();

	await testWithRefresh(page, async () => {
		await PrivacyPolicy.expect();
		await ShortPrivacyPolicy.expect();
		await Faqs.expect();
		await ThankYou.expect();
		await CTA.expect();
	});
});
