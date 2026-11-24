import type { Queries } from '$lib/interfaces/Conversation';
import type { UserDto } from '@crownshy/api-client/api';
import Conversation from '$lib/interfaces/Conversation';

const filters = ['shared', ...Conversation.statuses] as const;
export type Filter = (typeof filters)[number];

type Params = Pick<Queries, 'is_live' | 'is_complete' | 'exclude_owner_id'>;

function getParams(filter: Filter, userId: UserDto['id'] | undefined): Params {
	switch (filter) {
		case 'shared':
			return { exclude_owner_id: userId };
		default:
			return Conversation.getParams(filter);
	}
}

export default {
	filters,
	getParams
};
