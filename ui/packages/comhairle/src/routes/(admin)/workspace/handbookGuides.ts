import { ADMIN_GUIDES } from '$lib/admin_guides';

export type HandbookGuide = { key: string; title: string; href: string; topicTitles: string[] };

export const handbookGuides: HandbookGuide[] = Object.values(ADMIN_GUIDES).map((guide) => ({
	key: guide.key,
	title: guide.title,
	href: `/admin/info/how-to/${guide.key}`,
	topicTitles: guide.topics.map((topic) => topic.navLabel)
}));
