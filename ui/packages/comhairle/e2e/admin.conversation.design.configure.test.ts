import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { DraggableList, Switch, Textbox, Tiptap } from './utils/components';
import { testWithRefresh } from './utils';
import Design from './utils/navigation/Design';

test.beforeEach(async ({ page, cleanup }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Process design', undefined);
	const steps = await DraggableList<'learn'>({ page, cleanup });
	await steps.add('learn', 'new', 'Learn');
	await Design.openStep(page, 'New Learn Step');
	await Design.subtab(page, 'Configure');
});

test('Design/configure page', async ({ page, cleanup }) => {
	const Name = await Textbox(
		'Name',
		{ page, cleanup },
		{ resetTo: 'New Learn step', exact: true }
	);
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
});
