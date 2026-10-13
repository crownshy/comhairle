import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { LocalizedRegionDto } from '@crownshy/api-client/api';
import { createRegionEditor } from './regionEditor.svelte';

const mocks = vi.hoisted(() => ({
	ListRegions: vi.fn(),
	GetRegion: vi.fn(),
	GetRegionAreaLinks: vi.fn(),
	CreateRegion: vi.fn(),
	UpdateRegion: vi.fn(),
	SetRegionAreaLinks: vi.fn(),
	DeleteRegion: vi.fn(),
	saveTranslation: vi.fn()
}));

vi.mock('@crownshy/api-client/client', () => ({ apiClient: mocks }));
vi.mock('$lib/paraglide/runtime', () => ({ getLocale: () => 'en' }));
vi.mock('$lib/components/Translation/translationUtils', () => ({
	saveTranslation: mocks.saveTranslation
}));

const region: LocalizedRegionDto = {
	id: 'region-one',
	created_at: '2026-10-07T00:00:00Z',
	name: 'First region',
	description: 'Description',
	region_type: 'custom',
	official_id: null
};

function deferred<T>() {
	let resolve!: (value: T) => void;
	const promise = new Promise<T>((finish) => {
		resolve = finish;
	});
	return { promise, resolve };
}

beforeEach(() => {
	vi.resetAllMocks();
	mocks.ListRegions.mockResolvedValue({ records: [region], total: 1 });
	mocks.GetRegion.mockResolvedValue(region);
	mocks.GetRegionAreaLinks.mockResolvedValue({ area_ids: ['area-one'] });
	mocks.CreateRegion.mockResolvedValue(region);
	mocks.UpdateRegion.mockResolvedValue(region);
	mocks.SetRegionAreaLinks.mockResolvedValue(undefined);
	mocks.DeleteRegion.mockResolvedValue(undefined);
	mocks.saveTranslation.mockResolvedValue(undefined);
});

describe('region editor operation states', () => {
	it('starts idle and keeps draft visibility separate from operation state', () => {
		const editor = createRegionEditor();
		expect(editor.state).toBe('idle');
		expect(editor.editing).toBe(false);
		editor.create();
		expect(editor.editing).toBe(true);
		expect(editor.state).toBe('idle');
	});

	it('loads a region before making its draft editable', async () => {
		const editor = createRegionEditor();
		const selecting = editor.select(region);
		expect(editor.state).toBe('loading');
		expect(editor.loading).toBe(true);
		expect(editor.saving).toBe(false);
		expect(editor.editing).toBe(false);
		await selecting;
		expect(editor.state).toBe('idle');
		expect(editor.loading).toBe(false);
		expect(editor.editing).toBe(true);
		expect(editor.name).toBe(region.name);
		expect(editor.areaIds).toEqual(['area-one']);
		expect(editor.dirty).toBe(false);
	});

	it('reports a failed selection and recovers when another region loads', async () => {
		const editor = createRegionEditor();
		mocks.GetRegion.mockRejectedValueOnce(new Error('Failed'));
		await editor.select(region);
		expect(editor.state).toBe('error');
		expect(editor.loading).toBe(false);
		expect(editor.editing).toBe(false);
		expect(editor.error).toBe('Could not load this region.');
		await editor.select(region);
		expect(editor.state).toBe('idle');
		expect(editor.error).toBe('');
	});

	it('keeps the draft visible while saving and clears saved status on an edit', async () => {
		const editor = createRegionEditor();
		editor.create();
		editor.name = ' New region ';
		const saving = editor.save();
		expect(editor.state).toBe('saving');
		expect(editor.editing).toBe(true);
		expect(editor.loading).toBe(false);
		expect(editor.saved).toBe(false);
		await saving;
		expect(editor.state).toBe('saved');
		expect(editor.saving).toBe(false);
		expect(editor.saved).toBe(true);
		expect(editor.dirty).toBe(false);
		editor.name = 'Changed name';
		expect(editor.state).toBe('idle');
		expect(editor.saved).toBe(false);
		expect(editor.dirty).toBe(true);
	});

	it('retains a partially saved draft for retry after failure', async () => {
		const editor = createRegionEditor();
		editor.create();
		editor.name = 'New region';
		mocks.SetRegionAreaLinks.mockRejectedValueOnce(new Error('Failed'));
		await editor.save();
		expect(editor.state).toBe('error');
		expect(editor.saving).toBe(false);
		expect(editor.editing).toBe(true);
		expect(editor.name).toBe('New region');
		expect(editor.id).toBe(region.id);
		expect(editor.dirty).toBe(true);
		expect(editor.error).toContain('retained for retry');
		await editor.save();
		expect(editor.state).toBe('saved');
		expect(mocks.CreateRegion).toHaveBeenCalledOnce();
		expect(mocks.UpdateRegion).toHaveBeenCalledOnce();
	});

	it('does not let list failures or edits release the saving guard', async () => {
		const editor = createRegionEditor();
		editor.create();
		editor.name = 'New region';
		const creation = deferred<LocalizedRegionDto>();
		mocks.CreateRegion.mockReturnValueOnce(creation.promise);
		const saving = editor.save();
		mocks.ListRegions.mockRejectedValueOnce(new Error('Failed'));
		await editor.loadRegions();
		expect(editor.listing).toBe(false);
		expect(editor.state).toBe('saving');
		editor.name = 'Updated draft';
		editor.create();
		await editor.select(region);
		await editor.save();
		expect(editor.state).toBe('saving');
		expect(editor.name).toBe('Updated draft');
		expect(mocks.GetRegion).not.toHaveBeenCalled();
		expect(mocks.CreateRegion).toHaveBeenCalledOnce();
		creation.resolve(region);
		await saving;
		expect(editor.saving).toBe(false);
	});

	it('keeps saved status while refreshing the list and reports refresh failures', async () => {
		const editor = createRegionEditor();
		editor.create();
		editor.name = 'New region';
		const listing = deferred<{ records: LocalizedRegionDto[]; total: number }>();
		mocks.ListRegions.mockReturnValueOnce(listing.promise);
		const saving = editor.save();
		await vi.waitFor(() => expect(editor.listing).toBe(true));
		expect(editor.state).toBe('saved');
		expect(editor.saved).toBe(true);
		listing.resolve({ records: [region], total: 1 });
		await saving;
		mocks.ListRegions.mockRejectedValueOnce(new Error('Failed'));
		await editor.loadRegions();
		expect(editor.state).toBe('error');
		expect(editor.saved).toBe(false);
		expect(editor.editing).toBe(true);
		expect(editor.error).toContain('Could not load regions');
	});

	it('retains the draft on delete failure and returns to idle on successful deletion', async () => {
		const editor = createRegionEditor();
		await editor.select(region);
		mocks.DeleteRegion.mockRejectedValueOnce(new Error('Failed'));
		const removing = editor.remove();
		expect(editor.state).toBe('saving');
		await removing;
		expect(editor.state).toBe('error');
		expect(editor.editing).toBe(true);
		expect(editor.error).toBe('Could not delete this region.');
		await editor.remove();
		expect(editor.state).toBe('idle');
		expect(editor.editing).toBe(false);
		expect(editor.id).toBe(null);
		expect(editor.error).toBe('');
	});

	it('ignores stale region loads after a new draft starts', async () => {
		const editor = createRegionEditor();
		const loading = deferred<LocalizedRegionDto>();
		mocks.GetRegion.mockReturnValueOnce(loading.promise);
		const selecting = editor.select(region);
		editor.create();
		editor.name = 'New draft';
		loading.resolve(region);
		await selecting;
		expect(editor.state).toBe('idle');
		expect(editor.editing).toBe(true);
		expect(editor.name).toBe('New draft');
		expect(editor.id).toBe(null);
	});
});
