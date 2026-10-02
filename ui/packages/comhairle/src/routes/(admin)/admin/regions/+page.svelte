<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { invalidate } from '$app/navigation';
	import type { PageProps } from './$types';
	import type { MultiPolygon, Polygon } from 'geojson';
	import type { RegionAreaDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { AlertCircle, Check, FileUp, Map as MapIcon, Plus, Save, Search } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Pager } from '$lib/pagination';
	import { setPage } from '$lib/pagination';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { guardUnsavedChanges } from '$lib/utils/unsavedChangesGuard.svelte';
	import AreaMap from './AreaMap.svelte';
	import ImportAreas from './ImportAreas.svelte';
	import RegionsEditor from './RegionsEditor.svelte';
	import { boundarySchema } from './import';

	let props: PageProps = $props();
	let activeTab = $state('areas');
	let regionsVisited = $state(false);
	let areas = $derived(props.data.areas);
	let pageSize = $derived(props.data.pageSize);
	let selectedId = $state<string | null>(null);
	let selectedArea = $state<RegionAreaDto | null>(null);
	let name = $state('');
	let tagsText = $state('');
	let geometry = $state<Polygon | MultiPolygon | null>(null);
	let baseline = $state('');
	let importing = $state(false);
	let loading = $state(false);
	let saving = $state(false);
	let errorMessage = $state('');
	let saved = $state(false);

	const pageUrl = $derived(page.url);

	// Controlled search/tag inputs — debounced URL navigation
	let searchText = $state(page.url.searchParams.get('name') ?? '');
	let tagFilter = $state(page.url.searchParams.get('tag') ?? '');

	let searchTimer: ReturnType<typeof setTimeout>;
	function onSearchInput() {
		clearTimeout(searchTimer);
		searchTimer = setTimeout(() => pushFilters(), 400);
	}

	function pushFilters() {
		const url = new URL(page.url);
		if (searchText.trim()) {
			url.searchParams.set('name', searchText.trim());
		} else {
			url.searchParams.delete('name');
		}
		if (tagFilter) {
			url.searchParams.set('tag', tagFilter);
		} else {
			url.searchParams.delete('tag');
		}
		url.searchParams.delete('page');
		goto(url, { replaceState: true, keepFocus: true });
	}

	const tagOptions = $derived([...new Set(areas.records.flatMap((area) => area.tags))].sort());

	const parsedTags = $derived([
		...new Set(
			tagsText
				.split(',')
				.map((tag) => tag.trim())
				.filter(Boolean)
		)
	]);
	const currentSnapshot = $derived(JSON.stringify([name, parsedTags, geometry]));
	const isDirty = $derived(selectedArea !== null && currentSnapshot !== baseline);
	guardUnsavedChanges(() => isDirty || saving);

	async function selectArea(id: string) {
		if (saving || (isDirty && !window.confirm('Discard unsaved area changes?'))) return;
		selectedId = id;
		selectedArea = null;
		loading = true;
		errorMessage = '';
		saved = false;
		const response = await tryCatchAsync(() =>
			apiClient.GetRegionArea({ params: { region_area_id: id } })
		);
		loading = false;
		if (response.err !== null) {
			if (selectedId === id) errorMessage = 'Could not load this boundary.';
			return;
		}
		if (selectedId !== id) return;
		selectedArea = response.ok;
		name = response.ok.name ?? '';
		tagsText = response.ok.tags.join(', ');
		const parsedGeometry = boundarySchema.safeParse(response.ok.areaGeometry);
		geometry = parsedGeometry.success ? parsedGeometry.data : null;
		baseline = JSON.stringify([name, response.ok.tags, geometry]);
	}

	async function createArea() {
		if (saving || (isDirty && !window.confirm('Discard unsaved area changes?'))) return;
		selectedArea = null;
		saving = true;
		errorMessage = '';
		const response = await tryCatchAsync(() =>
			apiClient.CreateRegionArea({
				name: 'New area',
				tags: [],
				area_geometry: null,
				zip_prefix: null
			})
		);
		saving = false;
		if (response.err !== null) {
			errorMessage = 'Could not create a new area.';
			return;
		}
		await invalidate('admin:regions');
		await selectArea(response.ok.id);
	}

	async function saveArea() {
		if (!selectedArea || !isDirty) return;
		saving = true;
		errorMessage = '';
		saved = false;
		const response = await tryCatchAsync(() =>
			apiClient.UpdateRegionArea(
				{ name: name.trim() || null, tags: parsedTags, area_geometry: geometry },
				{ params: { region_area_id: (selectedArea as RegionAreaDto).id } }
			)
		);
		saving = false;
		if (response.err !== null) {
			errorMessage = 'Could not save changes. Check the boundary geometry and try again.';
			return;
		}
		selectedArea = response.ok;
		geometry = boundarySchema.safeParse(response.ok.areaGeometry).data ?? null;
		baseline = JSON.stringify([name, response.ok.tags, geometry]);
		tagsText = response.ok.tags.join(', ');
		saved = true;
		await invalidate('admin:regions');
	}

	async function refreshAreas() {
		await invalidate('admin:regions');
		importing = false;
		selectedId = null;
		selectedArea = null;
		geometry = null;
	}
</script>

<svelte:head>
	<title>Regions - Comhairle Admin</title>
</svelte:head>

<Tabs.Root
	bind:value={activeTab}
	onValueChange={(value) => {
		if (value === 'regions') regionsVisited = true;
	}}
	class="flex min-h-0 w-full flex-1 flex-col gap-0"
>
	<div class="bg-background grid gap-4 border-b px-5 py-4 md:px-8">
		<h1 class="text-2xl font-semibold">Regions</h1>
		<Tabs.List class="w-fit max-w-full" aria-label="Region management">
			<Tabs.Trigger value="areas" class="text-base">Geographic areas</Tabs.Trigger>
			<Tabs.Trigger value="regions" class="text-base">Regions</Tabs.Trigger>
		</Tabs.List>
	</div>
	<Tabs.Content value="areas" class="flex min-h-0 flex-1 flex-col data-[state=inactive]:hidden">
		<header
			class="bg-background flex flex-wrap items-center justify-between gap-4 border-b px-5 py-4 md:px-8"
		>
			<div class="flex items-center gap-3">
				<MapIcon class="text-primary size-6" />
				<div>
					<h2 class="text-xl font-semibold">Geographic areas</h2>
					<p class="text-muted-foreground text-base">{areas.total} boundaries</p>
				</div>
			</div>
			<div class="flex flex-wrap gap-2">
				<Button
					variant="outline"
					onclick={() => (importing = !importing)}
					disabled={saving || isDirty}
				>
					<FileUp class="size-4" />Import boundaries
				</Button>
				<Button onclick={createArea} disabled={saving}>
					<Plus class="size-4" />New area
				</Button>
			</div>
		</header>

		{#if importing}
			<ImportAreas onimport={refreshAreas} onclose={() => (importing = false)} />
		{/if}

		<div
			class="grid min-h-0 flex-1 grid-cols-1 lg:grid-cols-[minmax(260px,340px)_minmax(0,1fr)]"
		>
			<aside
				class="bg-background flex min-h-0 flex-col border-b lg:border-r lg:border-b-0"
				aria-label="Area list"
			>
				<div class="grid gap-3 border-b p-4">
					<label class="relative block">
						<Search class="text-muted-foreground absolute top-2.5 left-3 size-4" />
						<Input
							class="pl-9"
							bind:value={searchText}
							oninput={onSearchInput}
							placeholder="Search names or ZIP prefixes"
							aria-label="Search geographic areas"
						/>
					</label>
					<select
						class="bg-background h-9 rounded border px-3 text-base"
						bind:value={tagFilter}
						onchange={pushFilters}
						aria-label="Filter by tag"
					>
						<option value="">All tags</option>
						{#each tagOptions as tag (tag)}
							<option value={tag}>{tag}</option>
						{/each}
					</select>
				</div>
				<div class="min-h-0 flex-1 overflow-auto">
					{#each areas.records as area (area.id)}
						<button
							type="button"
							class="hover:bg-muted flex w-full flex-col gap-1 border-b px-4 py-3 text-left transition-colors {selectedId ===
							area.id
								? 'border-l-primary bg-muted border-l-2'
								: ''}"
							aria-current={selectedId === area.id ? 'true' : undefined}
							onclick={() => selectArea(area.id)}
						>
							<span class="font-medium"
								>{area.name || area.zipPrefix || 'Unnamed area'}</span
							>
							<span class="text-muted-foreground text-sm"
								>{area.tags.join(' · ') || 'No tags'}</span
							>
						</button>
					{:else}
						<p class="text-muted-foreground p-5">No areas match these filters.</p>
					{/each}
				</div>
				{#if areas.total > pageSize}
					<div class="border-t p-3">
						<Pager {pageSize} count={areas.total} url={pageUrl} />
					</div>
				{/if}
			</aside>

			<section class="flex min-h-[70svh] min-w-0 flex-col" aria-label="Area editor">
				{#if selectedArea}
					<div
						class="bg-background grid gap-4 border-b p-4 sm:grid-cols-[minmax(180px,1fr)_minmax(220px,1fr)_auto] sm:items-end md:px-6"
					>
						<label class="grid gap-2 font-medium"
							>Name
							<Input bind:value={name} aria-label="Area name" />
						</label>
						<label class="grid gap-2 font-medium"
							>Tags, comma separated
							<Input bind:value={tagsText} aria-label="Area tags" />
						</label>
						<Button onclick={saveArea} disabled={!isDirty || saving}>
							{#if saved}<Check class="size-4" />{:else}<Save class="size-4" />{/if}
							{saving ? 'Saving…' : saved ? 'Saved' : 'Save changes'}
						</Button>
					</div>
					{#if loading}<p role="status" class="p-4">Loading boundary…</p>{/if}
					{#key selectedArea.id}<AreaMap
							{geometry}
							onchange={(value) => (geometry = value)}
						/>{/key}
				{:else}
					<div
						class="flex min-h-[70svh] flex-1 items-center justify-center p-8 text-center"
					>
						<p class="text-muted-foreground text-base">
							{loading
								? 'Loading boundary…'
								: 'Select a region area to edit its boundary.'}
						</p>
					</div>
				{/if}
				{#if errorMessage}
					<p
						role="alert"
						class="bg-background text-destructive flex items-center gap-2 border-t p-4"
					>
						<AlertCircle class="size-4" />{errorMessage}
					</p>
				{/if}
			</section>
		</div>
	</Tabs.Content>
	<Tabs.Content value="regions" class="flex min-h-0 flex-1 flex-col data-[state=inactive]:hidden">
		{#if regionsVisited}<RegionsEditor areas={areas.records} />{/if}
	</Tabs.Content>
</Tabs.Root>
