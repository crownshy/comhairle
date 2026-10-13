<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { invalidate } from '$app/navigation';
	import type { PageProps } from './$types';
	import type { ComponentProps } from 'svelte';
	import type { MultiPolygon, Polygon } from 'geojson';
	import type { RegionAreaDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { AlertCircle, Check, FileUp, Map as MapIcon, Plus, Save } from 'lucide-svelte';
	import { useDebounce } from 'runed';
	import SearchBar from '$lib/components/SearchBar.svelte';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import * as Select from '$lib/components/ui/select';
	import * as Tabs from '$lib/components/ui/tabs';
	import BinaryTree from '$lib/data-structures/BinaryTree';
	import { Pager } from '$lib/pagination';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { key } from '$lib/utils/invalidationKey';
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
	let mapBoundary = $state.raw<ComponentProps<typeof AreaMap>['boundary'] | null>(null);
	let baseline = $state('');

	type State = 'idle' | 'loading' | 'saving' | 'saved' | 'error';
	let editorState = $state<State>('idle');
	let importAreaOpen = $state(false);
	let errorMessage = $state('');
	let requestId = 0;

	const pageUrl = $derived(page.url);

	// Controlled search/tag inputs, debounced URL navigation
	let searchText = $derived(page.url.searchParams.get('name') ?? '');
	let tagFilter = $derived(page.url.searchParams.get('tag') ?? '');

	const onSearchInput = useDebounce(pushFilters, 400);

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

	const tagOptions = $derived.by(() => {
		const binaryTree = new BinaryTree<string, string>();

		for (const area of areas.records) {
			for (const tag of area.tags) {
				binaryTree.insert(tag);
			}
		}

		return binaryTree.toArray();
	});

	const parsedTags = $derived(
		Array.from(
			new Set(
				tagsText
					.split(',')
					.map((tag) => tag.trim())
					.filter(Boolean)
			)
		)
	);
	const currentSnapshot = $derived(JSON.stringify([name, parsedTags, geometry]));
	const isDirty = $derived(selectedArea !== null && currentSnapshot !== baseline);
	guardUnsavedChanges(() => isDirty || editorState === 'saving');

	async function selectArea(id: string) {
		if (
			editorState === 'saving' ||
			(isDirty && !window.confirm('Discard unsaved area changes?'))
		)
			return;
		const currentRequest = ++requestId;
		selectedId = id;
		selectedArea = null;
		editorState = 'loading';
		errorMessage = '';
		const response = await tryCatchAsync(() =>
			apiClient.GetRegionArea({ params: { region_area_id: id } })
		);
		if (currentRequest !== requestId) return;
		if (response.err !== null) {
			errorMessage = 'Could not load this boundary.';
			editorState = 'error';
			return;
		}
		selectedArea = response.ok;
		name = response.ok.name ?? '';
		tagsText = response.ok.tags.join(', ');
		const parsedGeometry = boundarySchema.safeParse(response.ok.areaGeometry);
		const loadedGeometry = parsedGeometry.success ? parsedGeometry.data : null;
		geometry = loadedGeometry;
		mapBoundary = { geometry: loadedGeometry };
		baseline = JSON.stringify([name, response.ok.tags, geometry]);
		editorState = 'idle';
	}

	async function createArea() {
		if (
			editorState === 'saving' ||
			(isDirty && !window.confirm('Discard unsaved area changes?'))
		)
			return;
		requestId += 1;
		selectedId = null;
		selectedArea = null;
		editorState = 'saving';
		errorMessage = '';
		const response = await tryCatchAsync(() =>
			apiClient.CreateRegionArea({
				name: 'New area',
				tags: [],
				area_geometry: null,
				zip_prefix: null
			})
		);
		if (response.err !== null) {
			errorMessage = 'Could not create a new area.';
			editorState = 'error';
			return;
		}
		const refresh = await tryCatchAsync(() => invalidate(key('admin/regions')));
		if (refresh.err !== null) {
			errorMessage = 'Area created, but the list could not refresh. Reload the page.';
			editorState = 'error';
			return;
		}
		editorState = 'idle';
		await selectArea(response.ok.id);
	}

	async function saveArea() {
		if (!selectedArea || !isDirty || editorState === 'saving') return;
		const areaId = selectedArea.id;
		editorState = 'saving';
		errorMessage = '';
		const response = await tryCatchAsync(() =>
			apiClient.UpdateRegionArea(
				{ name: name.trim() || null, tags: parsedTags, area_geometry: geometry },
				{ params: { region_area_id: areaId } }
			)
		);
		if (response.err !== null) {
			errorMessage = 'Could not save changes. Check the boundary geometry and try again.';
			editorState = 'error';
			return;
		}
		selectedArea = response.ok;
		geometry = boundarySchema.safeParse(response.ok.areaGeometry).data ?? null;
		baseline = JSON.stringify([name, response.ok.tags, geometry]);
		tagsText = response.ok.tags.join(', ');
		const refresh = await tryCatchAsync(() => invalidate(key('admin/regions')));
		if (refresh.err !== null) {
			errorMessage = 'Changes saved, but the list could not refresh. Reload the page.';
			editorState = 'error';
			return;
		}
		editorState = 'saved';
	}

	async function refreshAreas() {
		const currentRequest = requestId;
		await invalidate(key('admin/regions'));
		importAreaOpen = false;
		if (editorState === 'saving' || currentRequest !== requestId) return;
		requestId += 1;
		selectedId = null;
		selectedArea = null;
		geometry = null;
		if (mapBoundary) mapBoundary = { geometry: null };
		editorState = 'idle';
		errorMessage = '';
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
	{#if activeTab === 'areas'}
		<Tabs.Content
			value="areas"
			class="flex min-h-0 flex-1 flex-col data-[state=inactive]:hidden"
		>
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
						onclick={() => (importAreaOpen = !importAreaOpen)}
						disabled={editorState === 'saving' || isDirty}
					>
						<FileUp class="size-4" />Import boundaries
					</Button>
					<Button onclick={createArea} disabled={editorState === 'saving'}>
						<Plus class="size-4" />New area
					</Button>
				</div>
			</header>

			{#if importAreaOpen}
				<ImportAreas onimport={refreshAreas} onclose={() => (importAreaOpen = false)} />
			{/if}

			<div
				class="grid min-h-0 flex-1 grid-cols-1 lg:grid-cols-[minmax(260px,340px)_minmax(0,1fr)]"
			>
				<aside
					class="bg-background flex min-h-0 flex-col border-b lg:border-r lg:border-b-0"
					aria-label="Area list"
				>
					<div class="grid gap-3 border-b p-4">
						<SearchBar
							bind:value={searchText}
							oninput={onSearchInput}
							placeholder="Search names or ZIP prefixes"
							aria-label="Search geographic areas"
						/>
						<Select.Root
							type="single"
							value={tagFilter ? `tag:${tagFilter}` : 'all'}
							onValueChange={(value) => {
								tagFilter = value === 'all' ? '' : value.slice(4);
								pushFilters();
							}}
						>
							<Select.Trigger class="w-full text-base" aria-label="Filter by tag">
								{tagFilter || 'All tags'}
							</Select.Trigger>
							<Select.Content>
								<Select.Item value="all" class="text-base">All tags</Select.Item>
								{#each tagOptions as tag (tag)}
									<Select.Item value={`tag:${tag}`} class="text-base"
										>{tag}</Select.Item
									>
								{/each}
							</Select.Content>
						</Select.Root>
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
							<Button
								onclick={saveArea}
								disabled={!isDirty || editorState === 'saving'}
							>
								{#if editorState === 'saved'}<Check class="size-4" />{:else}<Save
										class="size-4"
									/>{/if}
								{#if editorState === 'saving'}
									Saving…
								{:else if editorState === 'saved'}
									Saved
								{:else}
									Save changes
								{/if}
							</Button>
						</div>
						{#if editorState === 'loading'}<p role="status" class="p-4">
								Loading boundary…
							</p>{/if}
					{:else}
						<div
							class={mapBoundary
								? 'p-4'
								: 'flex min-h-[70svh] flex-1 items-center justify-center p-8 text-center'}
						>
							<p
								role={editorState === 'loading' ? 'status' : undefined}
								class="text-muted-foreground text-base"
							>
								{editorState === 'loading'
									? 'Loading boundary…'
									: 'Select a region area to edit its boundary.'}
							</p>
						</div>
					{/if}
					{#if mapBoundary}
						<AreaMap
							boundary={mapBoundary}
							disabled={selectedArea === null}
							onchange={(value) => (geometry = value)}
						/>
					{/if}
					{#if editorState === 'error'}
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
	{/if}
	{#if activeTab === 'regions'}
		<Tabs.Content
			value="regions"
			class="flex min-h-0 flex-1 flex-col data-[state=inactive]:hidden"
		>
			{#if regionsVisited}<RegionsEditor areas={areas.records} />{/if}
		</Tabs.Content>
	{/if}
</Tabs.Root>
