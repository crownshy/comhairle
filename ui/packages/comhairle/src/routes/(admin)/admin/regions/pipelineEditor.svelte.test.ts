import { beforeEach, describe, expect, it, vi } from 'vitest';
import type { Pipeline, PipelineDefinition, ReconciliationCounts } from '@crownshy/api-client/api';
import { createPipelineEditor } from './pipelineEditor.svelte';

const mocks = vi.hoisted(() => ({
	ListRegionAreaPipelines: vi.fn(),
	GetRegionAreaPipelinePreview: vi.fn(),
	CreateRegionAreaPipeline: vi.fn(),
	ReplaceRegionAreaPipelineDraft: vi.fn(),
	PreviewRegionAreaPipeline: vi.fn(),
	ActivateRegionAreaPipeline: vi.fn(),
	DeactivateRegionAreaPipeline: vi.fn(),
	DeleteRegionAreaPipeline: vi.fn(),
	axios: { post: vi.fn() }
}));
vi.mock('@crownshy/api-client/client', () => ({ apiClient: mocks }));

const definition: PipelineDefinition = {
	name: 'Counties',
	source: { type: 'https', url: 'https://example.org/counties.geojson' },
	external_id_field: 'CODE',
	area_name_field: 'NAME',
	primary_locale: 'en',
	region_name_fields: { en: 'NAME' },
	tags: { fixed: ['county'], rules: [] },
	interval_minutes: 1440,
	attribution: 'Open boundaries',
	repair_invalid_geometry: false
};
const pipeline: Pipeline = {
	id: 'f77abc4c-c26c-4c85-8af4-fc24b0391344',
	draft_definition: definition,
	status: 'draft',
	run_status: 'unscheduled',
	generation: 0,
	created_at: '2026-11-25T00:00:00Z',
	updated_at: '2026-11-25T00:00:00Z'
};
const preview: ReconciliationCounts = {
	created: 1,
	updated: 0,
	unchanged: 0,
	retired: 0,
	restored: 0,
	created_ids: ['county-1'],
	updated_ids: [],
	retired_ids: [],
	restored_ids: [],
	repaired: [],
	unmatched_ids: [],
	tag_changes: []
};

beforeEach(() => {
	vi.resetAllMocks();
	mocks.ListRegionAreaPipelines.mockResolvedValue([pipeline]);
	mocks.GetRegionAreaPipelinePreview.mockResolvedValue(preview);
	mocks.CreateRegionAreaPipeline.mockImplementation(async (draft) => ({
		...pipeline,
		draft_definition: draft
	}));
	mocks.ReplaceRegionAreaPipelineDraft.mockImplementation(async (draft) => ({
		...pipeline,
		draft_definition: draft
	}));
	mocks.PreviewRegionAreaPipeline.mockResolvedValue(preview);
	mocks.ActivateRegionAreaPipeline.mockResolvedValue({
		...pipeline,
		status: 'active',
		active_definition: definition
	});
	mocks.DeactivateRegionAreaPipeline.mockResolvedValue({ ...pipeline, status: 'deactivated' });
});

describe('pipeline draft publication', () => {
	it('opens a pipeline from the reactive list without mutating its saved definition', async () => {
		const editor = createPipelineEditor();
		await editor.load();
		await editor.select(editor.pipelines[0]);
		editor.definition.name = 'Local draft';
		expect(editor.pipelines[0].draft_definition.name).toBe('Counties');
		expect(editor.dirty).toBe(true);
	});

	it('blocks preview and activation for unsaved field and rule edits', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		expect(editor.canActivate).toBe(true);
		editor.definition.name = 'Changed';
		expect(editor.dirty).toBe(true);
		await editor.generatePreview();
		await editor.activate();
		expect(mocks.PreviewRegionAreaPipeline).not.toHaveBeenCalled();
		expect(mocks.ActivateRegionAreaPipeline).not.toHaveBeenCalled();
		editor.definition.name = definition.name;
		editor.rulesText = '[invalid';
		expect(editor.dirty).toBe(true);
		expect(editor.canActivate).toBe(false);
		await editor.save();
		expect(mocks.ReplaceRegionAreaPipelineDraft).not.toHaveBeenCalled();
		expect(editor.error).toBeTruthy();
		expect(editor.rulesText).toBe('[invalid');
	});

	it('saves typed tag rules and requires a new preview before publishing', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		const rules = [
			{ conditions: [{ operator: 'equals', field: 'LEVEL', value: 2 }], add: ['local'] }
		];
		editor.rulesText = JSON.stringify(rules);
		editor.fixedTagsText = 'county, public, county';
		await editor.save();
		expect(mocks.ReplaceRegionAreaPipelineDraft).toHaveBeenCalledWith(
			expect.objectContaining({ tags: { fixed: ['county', 'public'], rules } }),
			{ params: { pipeline_id: pipeline.id } }
		);
		expect(editor.dirty).toBe(false);
		expect(editor.preview).toBeNull();
		expect(editor.canActivate).toBe(false);
		await editor.generatePreview();
		expect(editor.canActivate).toBe(true);
		await editor.activate();
		expect(editor.selected?.status).toBe('active');
		expect(editor.preview).toBeNull();
		expect(editor.canActivate).toBe(false);
	});

	it('retains failed draft edits and surfaces the API validation error', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		editor.definition.external_id_field = '';
		mocks.ReplaceRegionAreaPipelineDraft.mockRejectedValue({
			response: { data: { err: 'External ID required' } }
		});
		await editor.save();
		expect(editor.error).toBe('External ID required');
		expect(editor.definition.external_id_field).toBe('');
		expect(editor.dirty).toBe(true);
		expect(editor.busy).toBe(false);
	});

	it('clears a stale preview after publication fails', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		mocks.ActivateRegionAreaPipeline.mockRejectedValue({
			response: { data: { err: 'Preview is stale' } }
		});
		await editor.activate();
		expect(editor.error).toBe('Preview is stale');
		expect(editor.canActivate).toBe(false);
		expect(editor.preview).toBeNull();
		await editor.generatePreview();
		expect(editor.canActivate).toBe(true);
	});

	it('does not replace a local draft during status refresh or overlapping operations', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		editor.definition.name = 'Unsaved name';
		await editor.load();
		expect(mocks.ListRegionAreaPipelines).not.toHaveBeenCalled();
		let resolve!: (value: Pipeline) => void;
		mocks.ReplaceRegionAreaPipelineDraft.mockReturnValue(
			new Promise<Pipeline>((finish) => {
				resolve = finish;
			})
		);
		const saving = editor.save();
		expect(editor.busy).toBe(true);
		editor.create();
		await editor.select({ ...pipeline, id: 'other' });
		expect(editor.definition.name).toBe('Unsaved name');
		resolve({ ...pipeline, draft_definition: { ...definition, name: 'Unsaved name' } });
		await saving;
		expect(editor.dirty).toBe(false);
	});

	it('uploads a private named file and stages its identifier without publishing', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		const uploadId = 'c7580a40-56c8-43b5-976f-2ae9da96d2b7';
		mocks.axios.post.mockResolvedValue({ data: { upload_id: uploadId } });
		await editor.upload(new File(['{}'], 'counties.geojson'));
		const [url, form] = mocks.axios.post.mock.calls[0];
		expect(url).toBe('/region_area_pipelines/uploads');
		expect(form.get('file').name).toBe('counties.geojson');
		expect(editor.definition.source).toEqual({ type: 'uploaded_file', upload_id: uploadId });
		expect(editor.dirty).toBe(true);
		expect(editor.preview).toBeNull();
		expect(mocks.ActivateRegionAreaPipeline).not.toHaveBeenCalled();
	});

	it('retains the configured source when an upload fails', async () => {
		const editor = createPipelineEditor();
		await editor.select(pipeline);
		mocks.axios.post.mockRejectedValue(new Error('Offline'));
		await editor.upload(new File(['{}'], 'counties.geojson'));
		expect(editor.definition.source).toEqual(definition.source);
		expect(editor.error).toBeTruthy();
		await editor.upload(new File([], 'empty.geojson'));
		expect(editor.error).toContain('non-empty');
		expect(mocks.axios.post).toHaveBeenCalledTimes(1);
	});

	it('deactivates without deleting live entities and invalidates the preview', async () => {
		const editor = createPipelineEditor();
		await editor.select({ ...pipeline, status: 'active', active_definition: definition });
		await editor.deactivate();
		expect(editor.selected?.status).toBe('deactivated');
		expect(editor.preview).toBeNull();
		expect(editor.notice).toContain('retained');
	});

	it('deletes a pipeline, passing through whether to delete its areas', async () => {
		const editor = createPipelineEditor();
		await editor.load();
		await editor.select(pipeline);
		mocks.DeleteRegionAreaPipeline.mockResolvedValue({ deleted_areas: 3 });
		await editor.remove(true);
		expect(mocks.DeleteRegionAreaPipeline).toHaveBeenCalledWith(undefined, {
			params: { pipeline_id: pipeline.id },
			queries: { delete_areas: true }
		});
		expect(editor.pipelines).toEqual([]);
		expect(editor.selected).toBeNull();
		expect(editor.editing).toBe(false);
		expect(editor.notice).toContain('3 geographic areas');
	});
});
