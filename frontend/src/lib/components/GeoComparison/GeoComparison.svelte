<script lang="ts">
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as Card from '$lib/components/ui/card/index.js';
	import { FillLayer, GeoJSON, LineLayer, MapLibre } from 'svelte-maplibre';
	import type GeoJson from 'svelte-maplibre/GeoJSON.svelte.svelte';
	import { Spinner } from '../ui/spinner';
	import { schemeBlues } from 'd3-scale-chromatic';
	import { scaleQuantize } from 'd3-scale';

	let data: null | GeoJson = $state(null);
	let map: maplibregl.Map | undefined = $state();
	let loaded = $state(false);
	const colorScale = scaleQuantize().domain([0, 100]).range(schemeBlues[7]);

	$effect(() => {
		fetch('/geo/lads.geojson')
			.then((r) => r.json())
			.then((r) => {
				r.features = r.features.map((f) => ({
					...f,
					properties: { ...f.properties, color: colorScale(Math.random() * 100) }
				}));
				data = r;
			});
	});
</script>

<Card.Root class="min-w-[400px]">
	<Card.Header>
		<Card.Title>Geographical Comparison</Card.Title>
	</Card.Header>
	<Card.Content>
		{#if data}
			<MapLibre
				bind:map
				bind:loaded
				style="https://basemaps.cartocdn.com/gl/positron-gl-style/style.json"
				class="min-h-[400px] w-[full]"
				standardControls
				center={[-4.177, 57.021]}
				zoom={5}
			>
				<GeoJSON id="states" {data}>
					<FillLayer
						paint={{
							'fill-color': ['get', 'color'],
							'fill-opacity': 0.5
						}}
						beforeLayerType="symbol"
						manageHoverState
					/>
					<LineLayer
						layout={{ 'line-cap': 'round', 'line-join': 'round' }}
						paint={{ 'line-color': '#f0f0f0', 'line-width': 1, 'line-opacity': 0.5 }}
						beforeLayerType="symbol"
					/>
				</GeoJSON>
			</MapLibre>
		{:else}
			<Spinner />
		{/if}
	</Card.Content>
	<Card.Footer>
		<div class="flex w-full items-start gap-2 text-sm">
			<div class="grid gap-2">
				<div class="text-muted-foreground flex items-center gap-2 leading-none">
					Location comparison
				</div>
			</div>
		</div>
	</Card.Footer>
</Card.Root>
