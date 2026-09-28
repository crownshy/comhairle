<script lang="ts">
	import { onMount } from 'svelte';
	import type { GeoJSONSource, Map as MaplibreMap, MapMouseEvent } from 'maplibre-gl';
	import type { RegionAreaDto } from '@crownshy/api-client/api';
	import { apiClient } from '@crownshy/api-client/client';
	import { Scan } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import * as Menu from '$lib/components/ui/dropdown-menu';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { areaFeatures } from './selection';
	import { positionsFromCoordinates } from './import';
	import mapLibreWorkerUrl from 'maplibre-gl/dist/maplibre-gl-worker.mjs?worker&url';
	import 'maplibre-gl/dist/maplibre-gl.css';

	type Props = {
		selectedIds: string[];
		disabled: boolean;
		ontoggle: (id: string) => void;
	};
	let { selectedIds, disabled, ontoggle }: Props = $props();
	let container: HTMLDivElement;
	let map: MaplibreMap | undefined;
	let ready = $state(false);
	let errorMessage = $state('');
	let status = $state('Loading boundaries...');
	/** Areas loaded by id, so the current selection always stays drawable. */
	const areaCache = new Map<string, RegionAreaDto>();
	let viewportAreas: RegionAreaDto[] = [];
	let matches = $state<RegionAreaDto[]>([]);
	let menuOpen = $state(false);
	let anchor = $state<{ getBoundingClientRect: () => DOMRect } | null>(null);
	let requestId = 0;
	let viewportRequestId = 0;
	let refreshTimer: ReturnType<typeof setTimeout>;
	let fitted = false;
	let disposed = false;

	/** Areas below this share of the visible rectangle are too small to be worth drawing. */
	const MIN_AREA_RATIO = 0.0002;
	const VIEWPORT_LIMIT = 1000;
	const ID_CHUNK = 100;
	const REFRESH_DELAY_MS = 1000;

	$effect(() => {
		const ids = [...selectedIds];
		if (!ready || !map) return;
		map.setFilter('selected-areas', ['in', ['get', 'id'], ['literal', ids]]);
		void loadSelected(ids);
	});

	$effect(() => {
		if (disabled) {
			menuOpen = false;
			requestId += 1;
		}
	});

	function selectedAreas(): RegionAreaDto[] {
		return selectedIds
			.map((id) => areaCache.get(id))
			.filter((area): area is RegionAreaDto => area !== undefined);
	}

	function render() {
		const selected = selectedAreas();
		const shown = new Set(selected.map((area) => area.id));
		const source = map?.getSource('areas');
		if (!source || source.type !== 'geojson') return;
		const geojsonSource = source as GeoJSONSource;
		geojsonSource.setData(
			areaFeatures([...selected, ...viewportAreas.filter((area) => !shown.has(area.id))])
		);
	}

	/** Normalise the map viewport to one WGS84 rectangle the API can filter on. */
	function viewportQueries() {
		const bounds = map?.getBounds();
		if (!bounds) return null;
		const south = Math.max(-89, bounds.getSouth());
		const north = Math.min(89, bounds.getNorth());
		const span = Math.min(358, bounds.getEast() - bounds.getWest());
		const west = ((((bounds.getWest() + 180) % 360) + 360) % 360) - 180;
		const east = west + span;
		if (![south, north, west, east].every(Number.isFinite) || north <= south) return null;
		const [minLng, maxLng] = east > west && east <= 180 ? [west, east] : [-179, 179];
		return {
			viewport_min_lng: minLng,
			viewport_min_lat: south,
			viewport_max_lng: maxLng,
			viewport_max_lat: north
		};
	}

	async function loadSelected(ids: string[]) {
		const missing = ids.filter((id) => !areaCache.has(id));
		if (!missing.length) {
			render();
			return;
		}
		const chunks: string[][] = [];
		for (let start = 0; start < missing.length; start += ID_CHUNK) {
			chunks.push(missing.slice(start, start + ID_CHUNK));
		}
		const response = await tryCatchAsync(() =>
			Promise.all(
				chunks.map((chunk) =>
					apiClient.ListRegionAreas({
						queries: { include_geometry: true, ids: chunk.join(','), limit: ID_CHUNK }
					})
				)
			)
		);
		if (disposed) return;
		if (response.err !== null) {
			errorMessage = 'Could not load the selected boundaries. Try again.';
			return;
		}
		for (const page of response.ok) {
			for (const area of page.records) areaCache.set(area.id, area);
		}
		render();
		if (!fitted && selectedIds.length) {
			fitted = true;
			fit();
		}
	}

	async function refresh() {
		const queries = viewportQueries();
		if (!ready || !queries) return;
		const currentRequest = ++viewportRequestId;
		status = 'Loading boundaries...';
		const response = await tryCatchAsync(() =>
			apiClient.ListRegionAreas({
				queries: {
					include_geometry: true,
					limit: VIEWPORT_LIMIT,
					min_area_ratio: MIN_AREA_RATIO,
					...queries
				}
			})
		);
		if (disposed || currentRequest !== viewportRequestId) return;
		if (response.err !== null) {
			status = '';
			errorMessage = 'Could not load boundaries for this view. Try again.';
			return;
		}
		viewportAreas = response.ok.records;
		status =
			response.ok.total > viewportAreas.length
				? 'Zoom in to see the remaining boundaries.'
				: '';
		render();
	}

	function scheduleRefresh() {
		clearTimeout(refreshTimer);
		refreshTimer = setTimeout(() => void refresh(), REFRESH_DELAY_MS);
	}

	function fit() {
		const selected = selectedAreas();
		const features = areaFeatures(selected);
		const positions = features.features.flatMap((feature) =>
			positionsFromCoordinates(feature.geometry.coordinates)
		);
		if (!map || !positions.length) return;
		const bounds: [number, number, number, number] = [Infinity, Infinity, -Infinity, -Infinity];
		for (const [longitude, latitude] of positions) {
			bounds[0] = Math.min(bounds[0], longitude);
			bounds[1] = Math.min(bounds[1], latitude);
			bounds[2] = Math.max(bounds[2], longitude);
			bounds[3] = Math.max(bounds[3], latitude);
		}
		map.fitBounds(bounds, { padding: 40, maxZoom: 13, duration: 0 });
	}

	async function selectAt(event: MapMouseEvent) {
		if (disabled || !ready) return;
		const currentRequest = ++requestId;
		menuOpen = false;
		errorMessage = '';
		status = 'Finding areas...';
		const rectangle = container.getBoundingClientRect();
		const left = rectangle.left + event.point.x;
		const top = rectangle.top + event.point.y;
		const response = await tryCatchAsync(() =>
			apiClient.IntersectingRegionAreas({
				queries: { longitude: event.lngLat.wrap().lng, latitude: event.lngLat.lat }
			})
		);
		if (disposed || disabled || currentRequest !== requestId) return;
		status = '';
		if (response.err !== null) {
			errorMessage = 'Could not find areas at this location. Try again.';
			return;
		}
		matches = response.ok.records;
		if (matches.length === 1) {
			ontoggle(matches[0].id);
		} else if (matches.length > 1) {
			anchor = { getBoundingClientRect: () => new DOMRect(left, top, 0, 0) };
			menuOpen = true;
		} else {
			status = 'No areas at this location.';
		}
	}

	onMount(() => {
		const result = tryCatchAsync(async () => {
			const maplibre = await import('maplibre-gl');
			if (disposed) return;
			maplibre.setWorkerUrl(mapLibreWorkerUrl);
			map = new maplibre.Map({
				container,
				style: 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json',
				center: [0, 30],
				zoom: 1,
				attributionControl: { compact: true }
			});
			map.addControl(new maplibre.NavigationControl(), 'top-right');
			map.on('error', () => {
				errorMessage =
					'Some map data could not load. Area selection is still available in the list.';
			});
			map.on('load', () => {
				if (!map || disposed) return;
				map.addSource('areas', {
					type: 'geojson',
					data: { type: 'FeatureCollection', features: [] }
				});
				map.addLayer({
					id: 'area-fill',
					type: 'fill',
					source: 'areas',
					paint: { 'fill-color': '#0891b2', 'fill-opacity': 0.12 }
				});
				map.addLayer({
					id: 'area-outline',
					type: 'line',
					source: 'areas',
					paint: { 'line-color': '#0e7490', 'line-width': 1.5 }
				});
				map.addLayer({
					id: 'selected-areas',
					type: 'fill',
					source: 'areas',
					filter: ['in', ['get', 'id'], ['literal', [...selectedIds]]],
					paint: { 'fill-color': '#ea580c', 'fill-opacity': 0.45 }
				});
				map.getCanvas().style.cursor = 'pointer';
				ready = true;
				status = '';
				void loadSelected([...selectedIds]);
				void refresh();
			});
			map.on('click', selectAt);
			map.on('movestart', () => {
				menuOpen = false;
				requestId += 1;
				status = '';
			});
			map.on('moveend', scheduleRefresh);
			map.on('zoomend', scheduleRefresh);
			const observer = new ResizeObserver(() => map?.resize());
			observer.observe(container);
			map.on('remove', () => observer.disconnect());
		});
		void result.then((response) => {
			if (response.err !== null && !disposed) {
				status = '';
				errorMessage =
					'Could not load the selection map. Area selection is still available in the list.';
			}
		});
		return () => {
			disposed = true;
			requestId += 1;
			viewportRequestId += 1;
			clearTimeout(refreshTimer);
			map?.remove();
		};
	});
</script>

<div
	class="relative min-h-[460px] flex-1 overflow-hidden border"
	aria-label="Region area selection map"
>
	<div bind:this={container} class="absolute! inset-0" aria-label="Area boundaries"></div>
	<Button
		class="bg-background absolute top-3 left-3"
		variant="outline"
		size="icon"
		title="Fit selected areas"
		aria-label="Fit selected areas"
		disabled={!ready || !selectedIds.length}
		onclick={fit}><Scan /></Button
	>
	{#if status}<p
			role="status"
			class="bg-background absolute bottom-10 left-3 max-w-[calc(100%-24px)] rounded border px-3 py-2 text-base"
		>
			{status}
		</p>{/if}
	{#if errorMessage}<p
			role="alert"
			class="bg-background text-destructive absolute right-3 bottom-10 left-3 rounded border p-3"
		>
			{errorMessage}
		</p>{/if}
</div>

<Menu.Root bind:open={menuOpen}>
	<Menu.Content
		customAnchor={anchor}
		side="right"
		align="start"
		sideOffset={8}
		collisionPadding={12}
		avoidCollisions
		class="max-h-[min(320px,var(--bits-dropdown-menu-content-available-height))] w-72 max-w-[calc(100vw-24px)] overflow-y-auto"
		onCloseAutoFocus={(event) => {
			event.preventDefault();
			map?.getCanvas().focus();
		}}
	>
		<Menu.Label class="text-base">Areas at this location</Menu.Label>
		{#each matches as area (area.id)}
			<Menu.CheckboxItem
				class="items-start text-base break-words"
				checked={selectedIds.includes(area.id)}
				onCheckedChange={() => ontoggle(area.id)}
				closeOnSelect={false}
				{disabled}
			>
				<span class="min-w-0"
					>{area.name || area.zipPrefix || 'Unnamed area'}<span
						class="text-muted-foreground block text-sm"
						>{area.tags.join(', ') || area.id}</span
					></span
				>
			</Menu.CheckboxItem>
		{/each}
	</Menu.Content>
</Menu.Root>
