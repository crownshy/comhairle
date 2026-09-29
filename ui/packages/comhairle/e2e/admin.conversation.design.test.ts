import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { DraggableList } from './utils/components';

test.describe('Design page', () => {
	test.beforeEach(async ({ page }) => {
		await login(page);
		await Conversation.open(page);
		await Conversation.openTab(page, 'Process Design', undefined);
	});

	test('Add learn step', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'new', 'Learn');
		await draggableList.expect.toInclude('a');
	});

	test('Add poll step', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'new', 'Poll');
		await draggableList.expect.toInclude('a');
	});

	test('Add survey step', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'new', 'Survey');
		await draggableList.expect.toInclude('a');
	});

	test('Add thinking space step', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'new', 'Individual view exploration');
		await draggableList.expect.toInclude('a');
	});

	test('Add prioritisation step', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'new', 'Prioritisation');
		await draggableList.expect.toInclude('a');
	});

	test('Add lived experience step', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'new', 'Lived experience');
		await draggableList.expect.toInclude('a');
	});

	test('Add step from tab bar', async ({ page, cleanup }) => {
		const draggableList = await DraggableList<'a'>({ page, cleanup });
		await draggableList.add('a', 'tab', 'Lived experience');
		await draggableList.expect.toInclude('a');
	});
});
