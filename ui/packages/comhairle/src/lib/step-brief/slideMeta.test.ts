import { describe, it, expect } from 'vitest';
import { stepMeta, toMetaToolConfig } from './slideMeta';

describe('stepMeta', () => {
	it('shows duration alone for a tool with no count', () => {
		const items = stepMeta({ type: 'learn' });
		expect(items).toHaveLength(1);
		expect(items[0].kind).toBe('duration');
	});

	it('adds an opinion count for polis', () => {
		const items = stepMeta({ type: 'polis', required_votes: 5 });
		expect(items.map((item) => item.kind)).toEqual(['duration', 'count']);
		expect(items[1].label).toContain('5');
	});

	it('adds a follow-up count for thinking space', () => {
		const items = stepMeta({ type: 'thinkingspace', follow_up_rounds_count: 2 });
		expect(items[1].label).toContain('2');
	});

	it('singularises a count of one', () => {
		const items = stepMeta({ type: 'polis', required_votes: 1 });
		expect(items[1].label).not.toContain('opinions');
	});

	it('omits a count that is absent, null or zero', () => {
		expect(stepMeta({ type: 'polis' })).toHaveLength(1);
		expect(stepMeta({ type: 'polis', required_votes: null })).toHaveLength(1);
		expect(stepMeta({ type: 'polis', required_votes: 0 })).toHaveLength(1);
	});

	it('does not read another tool config count onto the wrong tool', () => {
		expect(stepMeta({ type: 'learn', required_votes: 5 })).toHaveLength(1);
	});

	it('returns nothing for an unknown or missing tool config', () => {
		expect(stepMeta(null)).toEqual([]);
		expect(stepMeta({ type: 'nonsense' })).toEqual([]);
	});
});

describe('toMetaToolConfig', () => {
	it('picks the fields the meta line reads', () => {
		const result = toMetaToolConfig({ type: 'polis', required_votes: 5, poll_id: 'x' });
		expect(result).toEqual({
			type: 'polis',
			required_votes: 5,
			follow_up_rounds_count: null
		});
	});

	it('nulls a count that is the wrong type', () => {
		expect(toMetaToolConfig({ type: 'polis', required_votes: '5' })?.required_votes).toBeNull();
	});

	it('returns null for a missing or non-object config', () => {
		expect(toMetaToolConfig(null)).toBeNull();
		expect(toMetaToolConfig(undefined)).toBeNull();
		expect(toMetaToolConfig('polis')).toBeNull();
	});

	it('round-trips into stepMeta', () => {
		const config = toMetaToolConfig({ type: 'thinkingspace', follow_up_rounds_count: 3 });
		expect(stepMeta(config)).toHaveLength(2);
	});
});
