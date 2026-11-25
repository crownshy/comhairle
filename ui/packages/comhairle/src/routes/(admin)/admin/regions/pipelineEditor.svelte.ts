import { apiClient } from '@crownshy/api-client/client';
import {
	UploadSourceResponse,
	TagRule,
	type Pipeline,
	type PipelineDefinition,
	type ReconciliationCounts
} from '@crownshy/api-client/api';
import { apiErrorMessage } from '$lib/utils/apiErrorMessage';
import { pluralise } from '$lib/utils/pluralise';
import { tryCatchAsync } from '$lib/utils/errorHandling';

function emptyDefinition(): PipelineDefinition {
	return {
		name: '',
		source: { type: 'https', url: '' },
		external_id_field: '',
		area_name_field: '',
		primary_locale: 'en',
		region_name_fields: { en: '' },
		tags: { fixed: [], rules: [] },
		interval_minutes: 1440,
		attribution: '',
		repair_invalid_geometry: false
	};
}

export function createPipelineEditor() {
	let pipelines = $state<Pipeline[]>([]);
	let selected = $state<Pipeline | null>(null);
	let definition = $state<PipelineDefinition>(emptyDefinition());
	let editing = $state(false);
	let baseline = $state('');
	let rulesText = $state('[]');
	let rulesBaseline = $state('[]');
	let fixedTagsText = $state('');
	let fixedTagsBaseline = $state('');
	let preview = $state<ReconciliationCounts | null>(null);
	let busy = $state(false);
	let error = $state('');
	let notice = $state('');
	const dirty = $derived(
		editing &&
			(JSON.stringify(definition) !== baseline ||
				rulesText !== rulesBaseline ||
				fixedTagsText !== fixedTagsBaseline)
	);
	const canActivate = $derived(selected !== null && preview !== null && !dirty && !busy);

	function accept(pipeline: Pipeline) {
		selected = pipeline;
		definition = structuredClone($state.snapshot(pipeline.draft_definition));
		baseline = JSON.stringify(definition);
		rulesText = JSON.stringify(pipeline.draft_definition.tags.rules ?? [], null, 2);
		rulesBaseline = rulesText;
		fixedTagsText = (pipeline.draft_definition.tags.fixed ?? []).join(', ');
		fixedTagsBaseline = fixedTagsText;
		pipelines = [...pipelines.filter((entry) => entry.id !== pipeline.id), pipeline];
	}

	async function perform<T>(operation: () => Promise<T>, fallback: string) {
		busy = true;
		error = '';
		notice = '';
		const result = await tryCatchAsync(operation);
		busy = false;
		if (result.err !== null) error = apiErrorMessage(result.err, fallback);
		return result;
	}

	return {
		get pipelines() {
			return pipelines;
		},
		get selected() {
			return selected;
		},
		get definition() {
			return definition;
		},
		set definition(value: PipelineDefinition) {
			definition = value;
		},
		get rulesText() {
			return rulesText;
		},
		set rulesText(value: string) {
			rulesText = value;
		},
		get fixedTagsText() {
			return fixedTagsText;
		},
		set fixedTagsText(value: string) {
			fixedTagsText = value;
		},
		get editing() {
			return editing;
		},
		get dirty() {
			return dirty;
		},
		get preview() {
			return preview;
		},
		get canActivate() {
			return canActivate;
		},
		get busy() {
			return busy;
		},
		get error() {
			return error;
		},
		get notice() {
			return notice;
		},
		create() {
			if (busy) return;
			selected = null;
			definition = emptyDefinition();
			baseline = '';
			rulesText = '[]';
			rulesBaseline = '[]';
			fixedTagsText = '';
			fixedTagsBaseline = '';
			preview = null;
			editing = true;
			error = '';
			notice = '';
		},
		async load() {
			if (busy || dirty) return;
			const result = await perform(
				() => apiClient.ListRegionAreaPipelines(),
				'Could not load pipelines. Try refreshing.'
			);
			if (result.err !== null) return;
			pipelines = result.ok;
			if (selected) {
				const current = pipelines.find((entry) => entry.id === selected?.id);
				if (current) await this.select(current);
			}
		},
		async select(pipeline: Pipeline) {
			if (busy) return;
			accept(pipeline);
			preview = null;
			editing = true;
			const result = await perform(
				() =>
					apiClient.GetRegionAreaPipelinePreview({
						params: { pipeline_id: pipeline.id }
					}),
				'Could not load the preview. Generate a new preview to continue.'
			);
			if (result.err === null) preview = result.ok;
		},
		async save() {
			if (busy || !dirty) return;
			const result = await perform(async () => {
				const rules = TagRule.array().parse(JSON.parse(rulesText));
				const draft = {
					...$state.snapshot(definition),
					tags: {
						fixed: fixedTagsText
							.split(',')
							.map((tag) => tag.trim())
							.filter((tag, index, tags) => tag && tags.indexOf(tag) === index),
						rules
					}
				};
				return selected
					? apiClient.ReplaceRegionAreaPipelineDraft(draft, {
							params: { pipeline_id: selected.id }
						})
					: apiClient.CreateRegionAreaPipeline(draft);
			}, 'Could not save this draft. Check the fields and try again.');
			if (result.err !== null) return;
			accept(result.ok);
			preview = null;
			notice = 'Draft saved. Generate a preview before publishing.';
		},
		async generatePreview() {
			if (busy || dirty || !selected) return;
			const id = selected.id;
			preview = null;
			const result = await perform(
				() =>
					apiClient.PreviewRegionAreaPipeline(undefined, { params: { pipeline_id: id } }),
				'Could not preview this pipeline. Check the source and mappings, then try again.'
			);
			if (result.err === null) preview = result.ok;
		},
		async activate() {
			if (!canActivate || !selected) return;
			const id = selected.id;
			preview = null;
			const result = await perform(
				() =>
					apiClient.ActivateRegionAreaPipeline(undefined, {
						params: { pipeline_id: id }
					}),
				'Could not publish this pipeline. Generate a fresh preview and try again.'
			);
			if (result.err !== null) return;
			accept(result.ok);
			notice = 'Pipeline published and activated.';
		},
		async deactivate() {
			if (busy || !selected || dirty) return;
			const id = selected.id;
			const result = await perform(
				() =>
					apiClient.DeactivateRegionAreaPipeline(undefined, {
						params: { pipeline_id: id }
					}),
				'Could not deactivate this pipeline. Try again.'
			);
			if (result.err !== null) return;
			accept(result.ok);
			preview = null;
			notice = 'Pipeline deactivated. Existing Regions and areas are retained.';
		},
		async remove(deleteAreas: boolean) {
			if (busy || !selected) return;
			const id = selected.id;
			const result = await perform(
				() =>
					apiClient.DeleteRegionAreaPipeline(undefined, {
						params: { pipeline_id: id },
						queries: { delete_areas: deleteAreas }
					}),
				'Could not delete this pipeline. Try again.'
			);
			if (result.err !== null) return;
			pipelines = pipelines.filter((entry) => entry.id !== id);
			selected = null;
			preview = null;
			editing = false;
			notice = deleteAreas
				? `Pipeline deleted with ${result.ok.deleted_areas} geographic ${pluralise(result.ok.deleted_areas, 'area')}.`
				: 'Pipeline deleted. Its Regions and areas are kept and can now be edited directly.';
		},
		async upload(file: File) {
			if (busy) return;
			if (!file.size || file.size > 64 * 1024 * 1024) {
				error = 'Choose a non-empty GeoJSON file up to 64 MiB.';
				return;
			}
			const form = new FormData();
			form.append('file', file);
			const result = await perform(async () => {
				// The generated multipart endpoint cannot represent a named file field.
				const response = await apiClient.axios.post<unknown>(
					'/region_area_pipelines/uploads',
					form
				);
				return UploadSourceResponse.parse(response.data);
			}, 'Could not upload this source. Try again.');
			if (result.err !== null) return;
			definition.source = { type: 'uploaded_file', upload_id: result.ok.upload_id };
			preview = null;
			notice = `Uploaded ${file.name}. Save the draft to use this source.`;
		}
	};
}
