import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { Switch, Textbox, Tiptap } from './utils/components';
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
	const Name = await Textbox('Name', { page, cleanup }, { resetTo: 'New Learn step' });
	const Description = Tiptap(0, { page, cleanup });
	const RevisitableStep = await Switch('Revisitable step', { page, cleanup });
	const RequiredStep = await Switch('Required step', { page, cleanup });

	await Name.write();
	await Description.write();
	await RevisitableStep.toggle();
	await RequiredStep.toggle();

	await testWithRefresh(page, async () => {
		await Name.expect();
		await Description.expect();
		await RevisitableStep.expect();
		await RequiredStep.expect();
	});

	cleanup(async () => {
		await page.getByRole('button', { name: 'Delete step' }).click();
		await sleep(1);
		await page.getByRole('button', { name: 'Delete step' }).nth(1).click();
		await sleep(1.5);
	});
});
