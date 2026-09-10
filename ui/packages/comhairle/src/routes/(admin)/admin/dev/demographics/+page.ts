import type { PageLoad } from './$types';

export const load: PageLoad = async ({ parent, depends }) => {
	depends('dev:demographics');
	const { api, user, ownedConversations, permittedConversations } = await parent();

	const questionsRes = await api
		.GetDemographicsQuestions({ queries: { limit: 100 } })
		.catch(() => ({ records: [], total: 0 }));

	const ownedIds = new Set(ownedConversations.records.map((c) => c.id));
	const conversations = [
		...ownedConversations.records,
		...permittedConversations.records.filter((c) => !ownedIds.has(c.id))
	];

	return {
		user,
		conversations,
		questions: questionsRes.records
	};
};
