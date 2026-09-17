import { test } from './utils/testing';
import Conversation from './utils/navigation/Conversation';
import Textboxes from './utils/inputs/Textboxes';
import { login } from './utils/auth';
import CollapisbleRichFields from './utils/inputs/CollapsibleRichField';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Configure', 'Content');
});

test('Configure/Content page', async ({ page, cleanup }) => {
	const collapsibleRichFields = CollapisbleRichFields(
		page,
		[
			['privacy_policy', 'Add privacy policy'],
			['short_privacy_policy', 'Add short privacy policy'],
			['faqs', 'Add faqs'],
			['thank_you', 'Add thank you message']
		],
		cleanup
	);
	await collapsibleRichFields.write('privacy_policy', ' ');
	await collapsibleRichFields.write('short_privacy_policy', ' ');
	await collapsibleRichFields.write('faqs', ' ');
	await collapsibleRichFields.write('thank_you', ' ');

	const textboxes = Textboxes(page, [['cta', 'Call to action']], cleanup);
	await textboxes.write('cta', '');

	await collapsibleRichFields.expect();
	await textboxes.expect();
});
