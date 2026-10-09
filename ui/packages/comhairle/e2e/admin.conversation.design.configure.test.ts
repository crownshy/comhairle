import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { Switches, Textboxes, Tiptap } from './utils/components';
import { sleep, testWithRefresh } from './utils';
import Design from './utils/navigation/Design';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Process Design', undefined);
	await Design.addStep(page, 'tab', 'Learn');
	await Design.openStep(page, 'New Learn Step');
	await Design.subtab(page, 'Configure');
	await sleep(1.5);
});

// FIX: Figure out why the test is failing when run at "full-speed" without the "--debug" flag
test('Design/configure page', async ({ page, cleanup }) => {
	const textboxes = Textboxes.new([['name', 'Name']], { page, cleanup });
	const tiptap = Tiptap({ page, cleanup });
	const switches = await Switches(
		[
			['revisitable_step', 'Revisitable step'],
			['required_step', 'Required step']
		],
		{ page, cleanup }
	);

	await textboxes.write('name', 'New Learn step');
	await tiptap.write();
	await switches.toggle('revisitable_step');
	await switches.toggle('required_step');

	await testWithRefresh(page, async () => {
		await textboxes.expect();
		await tiptap.expect();
		await switches.expect();
	});

	cleanup(async () => {
		await page.getByRole('button', { name: 'Delete step' }).click();
		await sleep(1);
		await page.getByRole('button', { name: 'Delete step' }).nth(1).click();
		await sleep(1.5);
	});
});
