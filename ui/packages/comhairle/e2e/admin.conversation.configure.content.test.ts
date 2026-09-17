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
	const collapsibleRichFields = new CollapisbleRichFields(page, [
		['privacy_policy', 'Add privacy policy'],
		['short_privacy_policy', 'Add short privacy policy'],
		['faqs', 'Add faqs'],
		['thank_you', 'Add thank you message']
	]);
	await collapsibleRichFields.write('privacy_policy', cleanup, ' ');
	await collapsibleRichFields.write('short_privacy_policy', cleanup, ' ');
	await collapsibleRichFields.write('faqs', cleanup, ' ');
	await collapsibleRichFields.write('thank_you', cleanup, ' ');

	const textboxes = new Textboxes(page, [['cta', 'Call to action']]);
	await textboxes.write('cta', cleanup, '');

	await collapsibleRichFields.expected();
	await textboxes.expected();
});
