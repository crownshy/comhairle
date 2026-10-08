<script lang="ts">
	import { onMount } from 'svelte';
	import type { RegionAreaDto } from '@crownshy/api-client/api';
	import { Plus, RefreshCw, Save, Trash2 } from 'lucide-svelte';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Checkbox } from '$lib/components/ui/checkbox';
	import { Input } from '$lib/components/ui/input';
	import { Textarea } from '$lib/components/ui/textarea';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import { createRegionEditor } from './regionEditor.svelte';
	import RegionSelectionMap from './RegionSelectionMap.svelte';

	type Props = { areas: RegionAreaDto[] };
	let { areas }: Props = $props();
	const editor = createRegionEditor();
	let search = $state('');
	let areaSearch = $state('');
	let selectedOnly = $state(false);
	const filteredRegions = $derived(
		editor.regions.filter((region) =>
			`${region.name} ${region.official_id ?? ''}`
				.toLocaleLowerCase()
				.includes(search.toLocaleLowerCase())
		)
	);
	const filteredAreas = $derived(
		areas.filter((area) => {
			if (selectedOnly && !editor.areaIds.includes(area.id)) return false;
			return `${area.name ?? ''} ${area.zipPrefix ?? ''} ${area.tags.join(' ')}`
				.toLocaleLowerCase()
				.includes(areaSearch.toLocaleLowerCase());
		})
	);
	guardUnsavedChanges(() => editor.dirty || editor.saving);

	function canDiscard() {
		return (
			!editor.saving && (!editor.dirty || window.confirm('Discard unsaved region changes?'))
		);
	}

	onMount(() => {
		void editor.loadRegions();
		return () => editor.dispose();
	});
</script>

<div class="grid min-h-0 flex-1 grid-cols-1 lg:grid-cols-[minmax(240px,300px)_minmax(0,1fr)]">
	<aside
		class="bg-background min-w-0 border-b lg:border-r lg:border-b-0"
		aria-label="Region list"
	>
		<div class="grid gap-3 border-b p-4">
			<div class="flex items-center justify-between gap-2">
				<h2 class="text-xl font-semibold">Regions</h2>
				<Button
					size="icon"
					variant="ghost"
					title="Refresh regions"
					aria-label="Refresh regions"
					disabled={editor.listing}
					onclick={editor.loadRegions}><RefreshCw class="size-4" /></Button
				>
			</div>
			<Input bind:value={search} placeholder="Search regions" aria-label="Search regions" />
			<Button
				disabled={editor.saving}
				onclick={() => {
					if (canDiscard()) editor.create();
				}}><Plus class="size-4" />New region</Button
			>
		</div>
		<div class="max-h-64 overflow-y-auto lg:max-h-[65svh]">
			{#each filteredRegions as region (region.id)}
				<Button
					variant={editor.id === region.id ? 'secondary' : 'ghost'}
					class="h-auto w-full justify-start rounded-none border-b px-4 py-3 text-left whitespace-normal"
					disabled={editor.saving}
					aria-current={editor.id === region.id ? 'true' : undefined}
					onclick={() => {
						if (canDiscard()) void editor.select(region);
					}}
				>
					<span class="min-w-0 break-words"
						><span class="block text-base font-medium">{region.name}</span><span
							class="text-muted-foreground block text-sm"
							>{region.region_type === 'official'
								? 'Official'
								: 'Custom'}{region.official_id
								? ` - ${region.official_id}`
								: ''}</span
						></span
					>
				</Button>
			{:else}
				<p class="text-muted-foreground p-4" role="status">
					{editor.listing ? 'Loading regions...' : 'No regions found.'}
				</p>
			{/each}
		</div>
	</aside>
	<section class="flex min-w-0 flex-col" aria-label="Region editor">
		{#if editor.error}<p role="alert" class="text-destructive border-b p-4">
				{editor.error}
			</p>{/if}
		{#if editor.editing}
			<form
				class="grid gap-4 border-b p-4 md:px-6"
				onsubmit={(event) => {
					event.preventDefault();
					void editor.save();
				}}
			>
				<fieldset disabled={editor.saving} class="grid min-w-0 gap-4 sm:grid-cols-2">
					<label class="grid min-w-0 gap-2 font-medium"
						>Name<Input
							required
							bind:value={editor.name}
							aria-label="Region name"
						/></label
					>
					<label class="grid gap-2 font-medium"
						>Type<select
							class="bg-background h-9 w-full rounded-md border px-3 text-base"
							bind:value={editor.regionType}
							aria-label="Region type"
							><option value="custom">Custom</option><option value="official"
								>Official</option
							></select
						></label
					>
					<label class="grid gap-2 font-medium sm:col-span-2"
						>Description<Textarea
							bind:value={editor.description}
							aria-label="Region description"
							rows={2}
						/></label
					>
					<label class="grid gap-2 font-medium"
						>Official ID<Input
							bind:value={editor.officialId}
							aria-label="Region official ID"
						/></label
					>
					<div class="flex flex-wrap items-end justify-end gap-2">
						{#if editor.id}<Button
								type="button"
								variant="outline"
								size="icon"
								title="Delete region"
								aria-label="Delete region"
								onclick={() => {
									if (
										window.confirm(
											'Delete this region? Its area boundaries will not be deleted.'
										)
									)
										void editor.remove();
								}}><Trash2 class="size-4" /></Button
							>{/if}
						<Button
							type="submit"
							disabled={!editor.name.trim() || (!editor.dirty && !!editor.id)}
							><Save class="size-4" />{editor.saving
								? 'Saving...'
								: 'Save region'}</Button
						>
					</div>
				</fieldset>
				{#if editor.saved}<p role="status" class="text-muted-foreground">
						Region saved.
					</p>{/if}
			</form>
			<div class="grid min-w-0 flex-1 grid-cols-1 xl:grid-cols-[minmax(0,1fr)_280px]">
				{#key editor.id ?? 'new'}<RegionSelectionMap
						selectedIds={editor.areaIds}
						disabled={editor.saving}
						ontoggle={editor.toggle}
					/>{/key}
				<fieldset
					disabled={editor.saving}
					class="flex min-w-0 flex-col border-t xl:border-t-0 xl:border-l"
					aria-label="Associated areas"
				>
					<div class="grid gap-3 border-b p-4">
						<h3 class="text-lg font-semibold">
							Associated areas <span class="text-muted-foreground"
								>({editor.areaIds.length})</span
							>
						</h3>
						<SearchBar
							bind:value={areaSearch}
							placeholder="Search areas or tags"
							aria-label="Search associated areas"
						/>
						<label class="flex items-center gap-2 text-base"
							><Checkbox bind:checked={selectedOnly} />Selected only</label
						>
					</div>
					<div class="max-h-[460px] overflow-y-auto">
						{#each filteredAreas as area (area.id)}
							<label
								class="hover:bg-muted flex cursor-pointer items-start gap-3 border-b px-4 py-3"
								><Checkbox
									class="mt-1"
									checked={editor.areaIds.includes(area.id)}
									onCheckedChange={() => editor.toggle(area.id)}
									aria-label={area.name || area.zipPrefix || 'Unnamed area'}
								/><span class="min-w-0 break-words"
									><span class="block text-base"
										>{area.name || area.zipPrefix || 'Unnamed area'}</span
									><span class="text-muted-foreground block text-sm"
										>{area.tags.join(', ')}</span
									></span
								></label
							>
						{:else}<p class="text-muted-foreground p-4">No matching areas.</p>{/each}
					</div>
				</fieldset>
			</div>
		{:else}
			<div
				class="text-muted-foreground flex min-h-[460px] items-center justify-center p-8 text-center"
				role="status"
			>
				{editor.loading ? 'Loading region...' : 'No region selected.'}
			</div>
		{/if}
	</section>
</div>
