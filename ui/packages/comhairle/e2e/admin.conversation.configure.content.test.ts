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
	const PrivacyPolicy = await CollapsibleRichField('privacy policy', { page, cleanup });
	const ShortPrivacyPolicy = await CollapsibleRichField('short privacy policy', {
		page,
		cleanup
	});
	const Faqs = await CollapsibleRichField('faqs', { page, cleanup });
	const ThankYou = await CollapsibleRichField('thank you message', { page, cleanup });
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
