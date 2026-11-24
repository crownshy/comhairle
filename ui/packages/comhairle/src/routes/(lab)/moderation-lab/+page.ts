import { key } from '$lib/utils/invalidationKey';
import type { UserActions } from '@crownshy/api-client/api';
import { CONVERSATION_ID, mockStatements } from './mockApi';
import type { PageLoad } from './$types';

// Browser only: the fake API and its data live in the client.
export const ssr = false;

export const load: PageLoad = ({ depends }) => {
	depends(key('admin/polis/statements'));
	const conversationActions: UserActions = {
		resourceType: 'conversation',
		resourceId: CONVERSATION_ID,
		actions: ['conversation_moderate', 'conversation_update', 'conversation_export']
	};
	return { conversationActions, statements: mockStatements() };
};
