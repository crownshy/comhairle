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
	it('groups the sections by stage, with Launch and Close as milestones', () => {
		const grouped = stages(draft).map((stage) => [
			stage.id,
			stage.sections.map((link) => link.name)
		]);

		expect(grouped).toEqual([
			['set-up', ['Configure', 'Process design', 'Learning Assistant', 'Events']],
			['launch', []],
			['engage', ['Recruit', 'Notify', 'Monitor']],
			['close', []],
			['results', ['Report']]
		]);
	});

	it('locks Engage, Close and Results before launch', () => {
		expect(stages(draft).map((stage) => stage.state)).toEqual([
			'current',
			'current',
			'locked',
			'locked',
			'locked'
		]);
		expect(stages(draft)[3].link?.lock).toBe('until-launch');
		expect(stages(draft)[1].link?.href).toBe('/admin/conversations/c1/launch');
		expect(section(draft, 'invites')?.lock).toBe('until-launch');
		expect(section(draft, 'configure')?.lock).toBeNull();
	});

	it('marks Set up and Launched done once live', () => {
		const [setUp, launch, engage, , results] = stages(live);

		expect(launch.label).toBe('Launched');
		expect([setUp.state, launch.state, engage.state, results.state]).toEqual([
			'done',
			'done',
			'current',
			'current'
		]);
		expect(section(live, 'invites')?.lock).toBeNull();
	});

	it('opens Close once live and marks it done once the conversation closes', () => {
		expect(stages(live)[3].state).toBe('current');
		expect(stages(live)[3].link?.lock).toBeNull();
		expect(stages(closed)[2].state).toBe('done');
		expect(stages(closed)[3]).toMatchObject({ label: 'Closed', state: 'done' });
	});

	it('adds Moderate to Engage only when there is a Polis step', () => {
		const engage = (hasPolisStep: boolean) =>
			conversationStages({
				conversation: live,
				pathname: '/admin',
				canAdmin: true,
				hasPolisStep
			})
				.find((stage) => stage.id === 'engage')
				?.sections.map((link) => link.name);

		expect(engage(false)).toEqual(['Recruit', 'Notify', 'Monitor']);
		expect(engage(true)).toEqual(['Recruit', 'Notify', 'Moderate', 'Monitor']);
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
