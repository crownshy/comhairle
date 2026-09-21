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
		[
			['privacy_policy', 'Add privacy policy'],
			['short_privacy_policy', 'Add short privacy policy'],
			['faqs', 'Add faqs'],
			['thank_you', 'Add thank you message']
		],
		{
			page,
			cleanup
		}
	);
	await collapsibleRichFields.write('privacy_policy', ' ');
	await collapsibleRichFields.write('short_privacy_policy', ' ');
	await collapsibleRichFields.write('faqs', ' ');
	await collapsibleRichFields.write('thank_you', ' ');

	const textboxes = Textboxes.new([['cta', 'Call to action']], { page, cleanup });
	await textboxes.write('cta', '');

	await collapsibleRichFields.expect();
	await textboxes.expect();
});
