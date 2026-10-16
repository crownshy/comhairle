import { describe, expect, it } from 'vitest';
import { conversationStages, type StageConversation } from './conversationStages';

const draft: StageConversation = { id: 'c1', isLive: false, isComplete: false };
const live: StageConversation = { id: 'c1', isLive: true, isComplete: false };
const closed: StageConversation = { id: 'c1', isLive: true, isComplete: true };

function stages(conversation: StageConversation, canAdmin = true, pathname = '/admin') {
	return conversationStages({ conversation, pathname, canAdmin });
}

function section(conversation: StageConversation, path: string, canAdmin = true) {
	return stages(conversation, canAdmin)
		.flatMap((stage) => stage.sections)
		.find((link) => link.path === path);
}

describe('conversationStages', () => {
	it('groups the sections by stage, with Launch as an empty milestone', () => {
		const grouped = stages(draft).map((stage) => [
			stage.id,
			stage.sections.map((link) => link.name)
		]);

		expect(grouped).toEqual([
			['set-up', ['Configure', 'Process design', 'Learning Assistant', 'Events']],
			['launch', []],
			['engage', ['Recruit', 'Notify']],
			['results', ['Monitor', 'Report']]
		]);
	});

	it('locks Engage and Results before launch', () => {
		expect(stages(draft).map((stage) => stage.state)).toEqual([
			'current',
			'current',
			'locked',
			'locked'
		]);
		expect(section(draft, 'invites')?.lock).toBe('until-launch');
		expect(section(draft, 'configure')?.lock).toBeNull();
	});

	it('marks Set up and Launched done once live', () => {
		const [setUp, launch, engage, results] = stages(live);

		expect(launch.label).toBe('Launched');
		expect([setUp.state, launch.state, engage.state, results.state]).toEqual([
			'done',
			'done',
			'current',
			'current'
		]);
		expect(section(live, 'invites')?.lock).toBeNull();
	});

	it('marks Engage done once the conversation closes', () => {
		expect(stages(closed)[2].state).toBe('done');
	});

	it('locks Notify for people without Admin while leaving Monitor open', () => {
		expect(section(live, 'notifications', false)?.lock).toBe('admin-only');
		expect(section(live, 'monitor', false)?.lock).toBeNull();
		expect(section(live, 'notifications', true)?.lock).toBeNull();
	});

	it('marks the section you are in, including its sub-pages', () => {
		const links = conversationStages({
			conversation: live,
			pathname: '/admin/conversations/c1/design/step/s1/setup',
			canAdmin: true
		}).flatMap((stage) => stage.sections);

		expect(links.filter((link) => link.active).map((link) => link.path)).toEqual(['design']);
	});
});
