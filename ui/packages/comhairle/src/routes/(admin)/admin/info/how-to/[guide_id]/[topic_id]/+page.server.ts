import { error } from '@sveltejs/kit';
import { ADMIN_GUIDES, ADMIN_GUIDE_NAV } from '$lib/admin_guides';
import type { PageServerLoad } from './$types';
import { HttpStatus } from '$lib/utils/constants';

// Every topic in reading order, across all guides, so "Next" can carry on
// from the last topic of one guide to the first topic of the next.
const ALL_TOPICS = ADMIN_GUIDE_NAV.flatMap((guide) =>
	guide.topics.map((topic) => ({ guideKey: guide.key, topic }))
);

export const load: PageServerLoad = ({ params }) => {
	if (!Object.hasOwn(ADMIN_GUIDES, params.guide_id)) {
		error(HttpStatus.NotFound, 'Guide not found');
	}

	const guide = ADMIN_GUIDES[params.guide_id];
	const topic = guide.topics.find((topic) => topic.key === params.topic_id);

	if (!topic) {
		error(HttpStatus.NotFound, 'Topic not found');
	}

	const index = ALL_TOPICS.findIndex(
		(entry) => entry.guideKey === guide.key && entry.topic.key === topic.key
	);
	const following = ALL_TOPICS[index + 1];
	const next = following
		? {
				guideKey: following.guideKey,
				topicKey: following.topic.key,
				title: following.topic.title
			}
		: null;

	return { guide, topic, next };
};
