<script lang="ts">
	import { onMount, untrack } from 'svelte';
	import type { FeatureCollection, Polygon, MultiPolygon } from 'geojson';
	import type { Map, IControl } from 'maplibre-gl';
	import type MapboxDraw from '@mapbox/mapbox-gl-draw';
	import { MousePointer2, Pentagon, Trash2, Undo2, Redo2, Scan, Spline } from 'lucide-svelte';
	import { Button } from '$lib/components/ui/button';
	import { tryCatchAsync } from '$lib/utils/errorHandling';
	import { multiPolygon, positionsFromCoordinates } from './import';
	import mapLibreWorkerUrl from 'maplibre-gl/dist/maplibre-gl-worker.mjs?worker&url';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import '@mapbox/mapbox-gl-draw/dist/mapbox-gl-draw.css';

	type DrawEventMap = Map & {
		on(type: string, listener: () => void): unknown;
	};

	type Props = {
		/** Replace this snapshot when an area loads, not when its geometry is edited. */
		boundary: { geometry: Polygon | MultiPolygon | null };
		disabled?: boolean;
		onchange: (geometry: MultiPolygon | null) => void;
	};
	let { boundary, disabled = false, onchange }: Props = $props();
	let container: HTMLDivElement;
	let map: Map | undefined;
	let draw: MapboxDraw | undefined;
	let ready = $state(false);
	let error = $state('');
	let selected = $state(false);
	let history = $state<string[]>([]);
	let cursor = $state(0);
	let canEdit = $derived(ready && !disabled);

	$effect(() => {
		const snapshot = boundary;
		if (!ready) return;
		untrack(() => loadBoundary(snapshot.geometry));
	});

	$effect(() => {
		if (ready && disabled) {
			draw?.changeMode('simple_select');
		}
	});

	function loadBoundary(geometry: Polygon | MultiPolygon | null) {
		if (!draw) return;
		draw.changeMode('simple_select');
		const features: FeatureCollection<Polygon> = {
			type: 'FeatureCollection',
			features: geometry
				? multiPolygon(geometry).coordinates.map((coordinates, index) => ({
						type: 'Feature',
						id: `polygon-${index}`,
						properties: {},
						geometry: { type: 'Polygon', coordinates }
					}))
				: []
		};
		draw.set(features);
		selected = false;
		history = [JSON.stringify(draw.getAll())];
		cursor = 0;
		fit();
	}

	function emitGeometry() {
		const polygons =
			draw?.getAll().features.flatMap((feature) => {
				if (feature.geometry.type === 'Polygon') return [feature.geometry.coordinates];
				if (feature.geometry.type === 'MultiPolygon') return feature.geometry.coordinates;
				return [];
			}) ?? [];
		onchange(polygons.length ? { type: 'MultiPolygon', coordinates: polygons } : null);
	}

	function recordChange() {
		if (!draw || !canEdit) return;
		const snapshot = JSON.stringify(draw.getAll());
		if (snapshot === history[cursor]) return;
		history = [...history.slice(0, cursor + 1), snapshot].slice(-50);
		cursor = history.length - 1;
		emitGeometry();
	}

	function restore(offset: number) {
		if (!draw || !canEdit) return;
		cursor += offset;
		draw.changeMode('simple_select');
		draw.set(JSON.parse(history[cursor]) as FeatureCollection);
		selected = false;
		emitGeometry();
	}

	function fit() {
		const positions =
			draw?.getAll().features.flatMap((feature) => {
				const coordinates =
					'coordinates' in feature.geometry ? feature.geometry.coordinates : null;
				return positionsFromCoordinates(coordinates);
			}) ?? [];
		if (!positions.length || !map) return;
		const bounds: [number, number, number, number] = [Infinity, Infinity, -Infinity, -Infinity];
		for (const point of positions) {
			bounds[0] = Math.min(bounds[0], point[0]);
			bounds[1] = Math.min(bounds[1], point[1]);
			bounds[2] = Math.max(bounds[2], point[0]);
			bounds[3] = Math.max(bounds[3], point[1]);
		}
		map.fitBounds(bounds, { padding: 50, maxZoom: 14, duration: 0 });
	}

	onMount(() => {
		let disposed = false;
		const result = tryCatchAsync(async () => {
			const [maplibre, { default: Draw }] = await Promise.all([
				import('maplibre-gl'),
				import('@mapbox/mapbox-gl-draw')
			]);
			if (disposed) return;
			maplibre.setWorkerUrl(mapLibreWorkerUrl);
			Object.assign(Draw.constants.classes, {
				CANVAS: 'maplibregl-canvas',
				CONTROL_BASE: 'maplibregl-ctrl',
				CONTROL_PREFIX: 'maplibregl-ctrl-',
				CONTROL_GROUP: 'maplibregl-ctrl-group',
				ATTRIBUTION: 'maplibregl-ctrl-attrib'
			});
			map = new maplibre.Map({
				container,
				style: 'https://basemaps.cartocdn.com/gl/positron-gl-style/style.json',
				center: [0, 30],
				zoom: 1,
				attributionControl: { compact: true }
			});
			draw = new Draw({ displayControlsDefault: false });
			map.addControl(draw as unknown as IControl, 'top-right');
			map.addControl(new maplibre.NavigationControl(), 'top-right');
			map.on('error', (event) => {
				if (!ready && !disposed) {
					error = event.error.message || 'Map style or worker could not be loaded.';
				}
			});
			map.on('load', () => {
				if (!draw || disposed) return;
				error = '';
				ready = true;
			});
			const drawEventMap = map as DrawEventMap;
			drawEventMap.on('draw.create', recordChange);
			drawEventMap.on('draw.update', recordChange);
			drawEventMap.on('draw.delete', recordChange);
			drawEventMap.on('draw.selectionchange', () => {
				selected = !!draw?.getSelectedIds().length;
			});
			const observer = new ResizeObserver(() => map?.resize());
			observer.observe(container);
			map.on('remove', () => observer.disconnect());
		});
		void result.then((response) => {
			if (response.err !== null && !disposed) {
				error =
					response.err instanceof Error
						? response.err.message
						: 'The map could not load.';
			}
		});
		return () => {
			disposed = true;
			map?.remove();
		};
	});
</script>

<div class="bg-background relative min-h-[460px] w-full flex-1 overflow-hidden border">
	<div
		bind:this={container}
		class="map-container absolute inset-0"
		aria-label="Area boundary map"
		inert={disabled}
	></div>
	<div
		class="bg-background absolute top-3 left-3 flex max-w-[calc(100%-80px)] flex-wrap gap-1 rounded border p-1 shadow-sm"
		role="toolbar"
		aria-label="Boundary editing"
	>
		<Button
			size="icon"
			variant="ghost"
			title="Select polygon"
			aria-label="Select polygon"
			disabled={!canEdit}
			onclick={() => draw?.changeMode('simple_select')}><MousePointer2 /></Button
		>
		<Button
			size="icon"
			variant="ghost"
			title="Edit vertices"
			aria-label="Edit vertices"
			disabled={!canEdit || !selected}
			onclick={() => {
				const featureId = draw?.getSelectedIds()[0];
				if (featureId) draw?.changeMode('direct_select', { featureId });
			}}><Spline /></Button
		>
		<Button
			size="icon"
			variant="ghost"
			title="Draw polygon"
			aria-label="Draw polygon"
			disabled={!canEdit}
			onclick={() => draw?.changeMode('draw_polygon')}><Pentagon /></Button
		>
		<Button
			size="icon"
			variant="ghost"
			title="Delete selected vertices or polygon"
			aria-label="Delete selected vertices or polygon"
			disabled={!canEdit}
			onclick={() => {
				draw?.trash();
				recordChange();
			}}><Trash2 /></Button
		>
		<Button
			size="icon"
			variant="ghost"
			title="Undo"
			aria-label="Undo"
			disabled={!canEdit || cursor === 0}
			onclick={() => restore(-1)}><Undo2 /></Button
		>
		<Button
			size="icon"
			variant="ghost"
			title="Redo"
			aria-label="Redo"
			disabled={!canEdit || cursor >= history.length - 1}
			onclick={() => restore(1)}><Redo2 /></Button
		>
		<Button
			size="icon"
			variant="ghost"
			title="Fit boundary"
			aria-label="Fit boundary"
			disabled={!canEdit}
			onclick={fit}><Scan /></Button
		>
	</div>
	{#if error}<p
			role="alert"
			class="bg-background text-destructive absolute right-3 bottom-10 left-3 rounded p-3"
		>
			{error}
		</p>{/if}
</div>
