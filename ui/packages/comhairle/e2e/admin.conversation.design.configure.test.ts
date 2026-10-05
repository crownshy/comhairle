import { login } from './utils/auth';
import Conversation from './utils/navigation/Conversation';
import { test } from './utils/testing';
import { DraggableList } from './utils/components';

test.beforeEach(async ({ page }) => {
	await login(page);
	await Conversation.open(page);
	await Conversation.openTab(page, 'Process Design', undefined);
});

test('Design/configure page', async ({ page, cleanup }) => {
	const draggableList = await DraggableList<'a'>({ page, cleanup });
	await draggableList.add('a', 'new', 'Learn');
	await draggableList.expect();
});
