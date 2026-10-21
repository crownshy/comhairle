import { describe, expect, it } from 'vitest';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import { conversationSections, type HandbookTopicRef } from '$lib/config/conversation-steps';
import { HANDBOOK_HOME, MILESTONE_GUIDES, sectionHeader } from './sectionHeader';

const draft = { id: 'c1', isLive: false, isComplete: false };
const live = { id: 'c1', isLive: true, isComplete: false };
const closed = { id: 'c1', isLive: true, isComplete: true };

const at = (path: string, conversation = live) =>
	sectionHeader(`/admin/conversations/c1/${path}`, conversation);

const guideHref = (path: string) => at(path)?.content.guide?.href;

describe('sectionHeader', () => {
	it('labels a section with its stage, name and description', () => {
		expect(at('invites')).toEqual({
			content: {
				eyebrow: 'Engage',
				title: 'Recruit',
				description: 'Bring people into the conversation by email, open link or in person.',
				guide: {
					href: '/admin/info/how-to/manage-a-conversation/recruit',
					label: 'Recruit participants'
				}
			},
			hasTabs: true,
			compact: false
		});
	});

	it('gives every section its stage label', () => {
		expect(at('configure/details')?.content.eyebrow).toBe('Set up');
		expect(at('notifications')?.content.eyebrow).toBe('Engage');
		expect(at('moderate')?.content.eyebrow).toBe('Engage');
		expect(at('moderate/step/s1')?.content.eyebrow).toBe('Engage');
		expect(at('report')?.content.eyebrow).toBe('Results');
	});

	it('names a workflow step and links back to the board', () => {
		const steps = [{ id: 's1', name: 'Warm up', toolType: 'learn' }];
		const header = sectionHeader('/admin/conversations/c1/design/step/s1/setup', live, steps);
		expect(header?.content.title).toBe('Warm up');
		expect(header?.content.eyebrow).toBe('Topic onboarding');
		expect(header?.content.back).toEqual({
			href: '/admin/conversations/c1/design',
			label: 'Process design'
		});
		expect(header?.hasTabs).toBe(true);
		expect(header?.compact).toBe(true);
	});

	it('still shows a step header when the step is missing', () => {
		expect(at('design/step/gone/setup')?.content.title).toBe('Unnamed step');
	});

	it('uses the section, not the tab, for pages under a section', () => {
		expect(at('configure/access')?.content.title).toBe('Configure');
		expect(at('events/e1')?.content.eyebrow).toBe('Set up');
		expect(at('moderate/step/s1')?.content.title).toBe('Moderate');
	});

	it('links a tab to its own topic when it has one', () => {
		expect(guideHref('configure/access')).toBe(
			'/admin/info/how-to/launch-a-conversation/configure-access'
		);
		expect(guideHref('configure/details')).toBe(
			'/admin/info/how-to/create-a-conversation/name-a-conversation'
		);
		expect(guideHref('configure/moderation-policy')).toBe(
			'/admin/info/how-to/advanced/moderation-in-depth'
		);
		expect(guideHref('configure/team')).toBe(
			'/admin/info/how-to/getting-started/roles-and-permissions'
		);
		expect(guideHref('design/step/s1/configure')).toBe(
			'/admin/info/how-to/design-the-engagement-process/configure-a-step'
		);
		expect(guideHref('moderate/step/s1')).toBe(
			'/admin/info/how-to/manage-a-conversation/moderate'
		);
	});

	it('knows which sections have a tab strip under the header', () => {
		expect(at('configure/details')?.hasTabs).toBe(true);
		expect(at('design')?.hasTabs).toBe(true);
		expect(at('monitor')?.hasTabs).toBe(false);
	});

	it('compacts the header on editor and step pages only', () => {
		expect(at('events/e1')?.compact).toBe(true);
		expect(at('design/step/s1/setup')?.compact).toBe(true);
		expect(at('design')?.compact).toBe(false);
		expect(at('events')?.compact).toBe(false);
		expect(at('events/new')?.compact).toBe(false);
		expect(at('configure/details')?.compact).toBe(false);
	});

	it('treats Launch and Close as milestones without a stage label', () => {
		expect(at('launch', draft)?.content).toMatchObject({ title: 'Launch' });
		expect(at('launch', draft)?.content.eyebrow).toBeUndefined();
		expect(at('launch', live)?.content.title).toBe('Launched');
		expect(at('launch', live)?.content.description).toBeDefined();
		expect(at('close', live)?.content.title).toBe('Close');
		expect(at('close', closed)?.content.title).toBe('Closed');
	});

	it('ignores trailing slashes and pages outside the conversation', () => {
		expect(at('monitor/')?.content.title).toBe('Monitor');
		expect(sectionHeader('/admin/conversations/c1', live)).toBeNull();
		expect(sectionHeader('/admin/conversations/other/monitor', live)).toBeNull();
		expect(at('unknown')).toBeNull();
	});

	it('gives every page a real Handbook topic', () => {
		const paths = [
			...conversationSections.map((s) => s.path),
			'design/step/s1/insights',
			'launch',
			'close'
		];
		for (const path of paths) {
			expect(at(path)?.content.guide, path).not.toEqual(HANDBOOK_HOME);
		}
	});

	it('only points at topics that exist in the Handbook', () => {
		const refs: [string, HandbookTopicRef | undefined][] = [
			...Object.entries(MILESTONE_GUIDES),
			...conversationSections.flatMap((s) => [
				[s.path, s.guide?.section] as [string, HandbookTopicRef | undefined],
				...Object.entries(s.guide?.tabs ?? {}).map(
					([tab, ref]) => [`${s.path}/${tab}`, ref] as [string, HandbookTopicRef]
				)
			])
		];
		for (const [page, ref] of refs) {
			if (!ref) continue;
			const topic = ADMIN_GUIDES[ref.guide]?.topics.find((t) => t.key === ref.topic);
			expect(topic, `${page}: ${ref.guide}/${ref.topic}`).toBeDefined();
		}
	});
});
