import { describe, expect, it } from 'vitest';
import { ADMIN_GUIDES } from '$lib/admin_guides';
import { HANDBOOK_HOME, PAGE_GUIDES, sectionHeader } from './sectionHeader';

const draft = { id: 'c1', isLive: false, isComplete: false };
const live = { id: 'c1', isLive: true, isComplete: false };
const closed = { id: 'c1', isLive: true, isComplete: true };

const at = (path: string, conversation = live) =>
	sectionHeader(`/admin/conversations/c1/${path}`, conversation);

describe('sectionHeader', () => {
	it('labels a section with its stage, name and description', () => {
		expect(at('invites')).toEqual({
			eyebrow: 'Engage',
			title: 'Recruit',
			description: 'Bring people into the conversation by email, open link or in person.',
			guide: {
				href: '/admin/info/how-to/manage-a-conversation/recruit',
				label: 'Recruit participants'
			},
			hasTabs: true,
			compact: false
		});
	});

	it('uses the section, not the tab, for pages under a section', () => {
		expect(at('configure/access')?.title).toBe('Configure');
		expect(at('design/step/s1/setup')?.title).toBe('Process design');
		expect(at('events/e1')?.eyebrow).toBe('Set up');
	});

	it('links a tab to its own topic when it has one', () => {
		expect(at('configure/access')?.guide.href).toBe(
			'/admin/info/how-to/launch-a-conversation/configure-access'
		);
		expect(at('configure/details')?.guide.href).toBe(
			'/admin/info/how-to/create-a-conversation/name-a-conversation'
		);
		expect(at('design/step/s1/configure')?.guide.href).toBe(
			'/admin/info/how-to/design-the-engagement-process/configure-a-step'
		);
	});

	it('falls back to the Handbook home when there is no topic yet', () => {
		expect(at('events')?.guide).toEqual(HANDBOOK_HOME);
		expect(at('learning-assistant')?.guide).toEqual(HANDBOOK_HOME);
	});

	it('knows which sections have a tab strip under the header', () => {
		expect(at('configure/details')?.hasTabs).toBe(true);
		expect(at('design')?.hasTabs).toBe(true);
		expect(at('monitor')?.hasTabs).toBe(false);
	});

	it('compacts the header on editor pages only', () => {
		expect(at('design/step/s1/setup')?.compact).toBe(true);
		expect(at('events/e1')?.compact).toBe(true);
		expect(at('design')?.compact).toBe(false);
		expect(at('events')?.compact).toBe(false);
		expect(at('events/new')?.compact).toBe(false);
		expect(at('configure/details')?.compact).toBe(false);
	});

	it('treats Launch and Close as milestones without a stage label', () => {
		expect(at('launch', draft)).toMatchObject({ title: 'Launch' });
		expect(at('launch', draft)?.eyebrow).toBeUndefined();
		expect(at('launch', live)).toMatchObject({ title: 'Launched', description: undefined });
		expect(at('close', live)?.title).toBe('Close');
		expect(at('close', closed)?.title).toBe('Closed');
	});

	it('ignores trailing slashes and pages outside the conversation', () => {
		expect(at('monitor/')?.title).toBe('Monitor');
		expect(sectionHeader('/admin/conversations/c1', live)).toBeNull();
		expect(sectionHeader('/admin/conversations/other/monitor', live)).toBeNull();
		expect(at('unknown')).toBeNull();
	});

	it('only points at topics that exist in the Handbook', () => {
		for (const [page, ref] of Object.entries(PAGE_GUIDES)) {
			const topic = ADMIN_GUIDES[ref.guide]?.topics.find((t) => t.key === ref.topic);
			expect(topic, `${page} → ${ref.guide}/${ref.topic}`).toBeDefined();
		}
	});
});
