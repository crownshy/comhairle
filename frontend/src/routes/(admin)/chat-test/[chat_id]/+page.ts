import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	return {
		chatId: params.chat_id
	};
};
