<script lang="ts">
	import type { PipelineDefinition } from '@crownshy/api-client/api';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import FileInput from '$lib/components/FileInput.svelte';

	type Props = {
		definition: PipelineDefinition;
		rulesText: string;
		fixedTagsText: string;
		busy: boolean;
		dirty: boolean;
		onsave: () => Promise<void>;
		onupload: (file: File) => Promise<void>;
	};
	let {
		definition = $bindable(),
		rulesText = $bindable(),
		fixedTagsText = $bindable(),
		busy,
		dirty,
		onsave,
		onupload
	}: Props = $props();
	let locale = $state('');
	let localeField = $state('');
	let mappingError = $state('');

	function addMapping() {
		const key = locale.trim();
		if (!key || !localeField.trim() || key in definition.region_name_fields) {
			mappingError = 'Use a new locale and a non-empty property name.';
			return;
		}
		definition.region_name_fields[key] = localeField.trim();
		locale = '';
		localeField = '';
		mappingError = '';
	}
</script>

<form
	onsubmit={(event) => {
		event.preventDefault();
		onsave();
	}}
	class="grid gap-6"
>
	<fieldset disabled={busy} class="grid min-w-0 gap-6">
		<legend class="mb-4 text-xl font-semibold">Draft configuration</legend>
		<div class="grid gap-4 md:grid-cols-2">
			<label class="grid gap-2"
				>Pipeline name<Input required bind:value={definition.name} /></label
			>
			<label class="grid gap-2"
				>Refresh interval (minutes)<Input
					type="number"
					min={1}
					max={4294967295}
					step={1}
					required
					bind:value={definition.interval_minutes}
				/></label
			>
		</div>
		<fieldset class="grid gap-3">
			<legend class="mb-2 font-semibold">GeoJSON source</legend>
			<p class="text-muted-foreground">
				Use a WGS84 FeatureCollection containing Polygon or MultiPolygon features.
			</p>
			<label class="grid gap-2"
				>Source type
				<select
					class="bg-background h-10 rounded border px-3"
					value={definition.source.type}
					onchange={(event) => {
						definition.source =
							event.currentTarget.value === 'https'
								? { type: 'https', url: '' }
								: { type: 'uploaded_file', upload_id: '' };
					}}
				>
					<option value="https">HTTPS URL</option>
					<option value="uploaded_file">Private file upload</option>
				</select>
			</label>
			{#if definition.source.type === 'https'}
				<label class="grid gap-2"
					>Source URL<Input
						type="url"
						pattern="https://.*"
						required
						placeholder="https://example.org/boundaries.geojson"
						bind:value={definition.source.url}
					/></label
				>
			{:else}
				<FileInput
					name="pipeline-source"
					accept=".geojson,.json"
					maxSize={64 * 1024 * 1024}
					disabled={busy}
					onfile={onupload}
				/>
				{#if definition.source.upload_id}
					<p class="text-muted-foreground break-all">
						Private source: {definition.source.upload_id}
					</p>
				{:else}
					<p class="text-muted-foreground">Upload a file before saving this draft.</p>
				{/if}
			{/if}
		</fieldset>
		<fieldset class="grid gap-4">
			<legend class="mb-2 font-semibold">Feature property mappings</legend>
			<p class="text-muted-foreground">
				Property names are case-sensitive. External identifiers must be unique and stable
				across refreshes.
			</p>
			<div class="grid gap-4 md:grid-cols-2">
				<label class="grid gap-2"
					>External identifier property<Input
						required
						bind:value={definition.external_id_field}
					/></label
				>
				<label class="grid gap-2"
					>Area name property<Input
						required
						bind:value={definition.area_name_field}
					/></label
				>
				<label class="grid gap-2"
					>Primary locale<Input
						required
						placeholder="en"
						bind:value={definition.primary_locale}
					/></label
				>
			</div>
			{#each Object.keys(definition.region_name_fields) as mappingLocale (mappingLocale)}
				<div class="flex items-end gap-3">
					<label class="grid flex-1 gap-2"
						>Region name property ({mappingLocale})<Input
							required
							bind:value={definition.region_name_fields[mappingLocale]}
						/></label
					>
					<Button
						type="button"
						variant="outline"
						aria-label={`Remove ${mappingLocale} mapping`}
						disabled={mappingLocale === definition.primary_locale}
						onclick={() => {
							delete definition.region_name_fields[mappingLocale];
						}}>Remove</Button
					>
				</div>
			{/each}
			<div class="grid items-end gap-3 md:grid-cols-[1fr_1fr_auto]">
				<label class="grid gap-2"
					>Additional locale<Input placeholder="ga" bind:value={locale} /></label
				>
				<label class="grid gap-2"
					>Region name property<Input
						placeholder="NAME_GA"
						bind:value={localeField}
					/></label
				>
				<Button type="button" variant="outline" onclick={addMapping}>Add mapping</Button>
			</div>
			{#if mappingError}<p role="alert" class="text-destructive">{mappingError}</p>{/if}
		</fieldset>
		<label class="grid gap-2"
			>Fixed tags (comma separated)
			<Input bind:value={fixedTagsText} />
		</label>
		<details class="rounded border p-4">
			<summary class="cursor-pointer font-semibold">Conditional tag rules (advanced)</summary>
			<p class="text-muted-foreground my-3">
				Rules match all their conditions. Operators: equals (value), in (values array),
				present (field only). Values can be strings, numbers, booleans or null.
			</p>
			<label class="grid gap-2"
				>Rules JSON<Textarea
					class="min-h-48 font-mono"
					bind:value={rulesText}
					spellcheck={false}
				/></label
			>
			<p class="text-muted-foreground mt-3">Example:</p>
			<pre class="mt-2 overflow-auto text-base">{JSON.stringify(
					[
						{
							conditions: [{ operator: 'equals', field: 'LEVEL', value: 'county' }],
							add: ['county']
						}
					],
					null,
					2
				)}</pre>
		</details>
		<label class="grid gap-2"
			>Attribution<Textarea required bind:value={definition.attribution} /></label
		>
		<label class="flex items-center gap-3"
			><input
				type="checkbox"
				class="size-4"
				bind:checked={definition.repair_invalid_geometry}
			/>Repair invalid geometry during import</label
		>
		<p class="text-muted-foreground">
			Saving only updates the draft. Preview and publish to change live Regions and areas.
		</p>
		<div>
			<Button
				type="submit"
				disabled={!dirty ||
					busy ||
					(definition.source.type === 'uploaded_file' && !definition.source.upload_id)}
				>Save draft</Button
			>
		</div>
	</fieldset>
</form>
