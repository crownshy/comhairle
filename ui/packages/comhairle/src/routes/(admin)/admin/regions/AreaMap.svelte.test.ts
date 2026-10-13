// @vitest-environment jsdom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { flushSync, mount, unmount } from 'svelte';
import type { ComponentProps } from 'svelte';
import type { FeatureCollection, Polygon } from 'geojson';
import AreaMap from './AreaMap.svelte';

const mocks = vi.hoisted(() => {
	const events = new Map<string, () => void>();
	let features: FeatureCollection = { type: 'FeatureCollection', features: [] };
	const map = {
		on: vi.fn((event: string, callback: () => void) => events.set(event, callback)),
		addControl: vi.fn(),
		fitBounds: vi.fn(),
		resize: vi.fn(),
		remove: vi.fn()
	};
	const draw = {
		set: vi.fn((value: FeatureCollection) => {
			features = JSON.parse(JSON.stringify(value));
			return [];
		}),
		getAll: vi.fn((): FeatureCollection => JSON.parse(JSON.stringify(features))),
		changeMode: vi.fn(),
		getSelectedIds: vi.fn(() => []),
		trash: vi.fn()
	};
	return {
		events,
		map,
		draw,
		Map: vi.fn(function () {
			return map;
		}),
		Draw: Object.assign(
			vi.fn(function () {
				return draw;
			}),
			{ constants: { classes: {} } }
		)
	};
});

vi.mock('maplibre-gl', () => ({
	Map: mocks.Map,
	NavigationControl: vi.fn(),
	setWorkerUrl: vi.fn()
}));
vi.mock('@mapbox/mapbox-gl-draw', () => ({ default: mocks.Draw }));

const polygon: Polygon = {
	type: 'Polygon',
	coordinates: [
		[
			[0, 0],
			[1, 0],
			[1, 1],
			[0, 0]
		]
	]
};
const otherPolygon: Polygon = {
	type: 'Polygon',
	coordinates: [
		[
			[10, 10],
			[11, 10],
			[11, 11],
			[10, 10]
		]
	]
};

let component: ReturnType<typeof mount> | undefined;

beforeEach(() => {
	vi.clearAllMocks();
	mocks.events.clear();
	vi.stubGlobal(
		'ResizeObserver',
		class {
			observe() {}
			disconnect() {}
		}
	);
});

afterEach(async () => {
	if (component) await unmount(component);
	component = undefined;
	document.body.replaceChildren();
	vi.unstubAllGlobals();
});

async function renderMap(props: ComponentProps<typeof AreaMap>, load = true) {
	component = mount(AreaMap, { target: document.body, props });
	flushSync();
	await vi.waitFor(() => expect(mocks.Map).toHaveBeenCalledOnce());
	if (load) {
		mocks.events.get('load')?.();
		flushSync();
	}
}

function button(label: string) {
	return document.querySelector<HTMLButtonElement>(`button[aria-label="${label}"]`);
}

describe('AreaMap boundary replacement', () => {
	it('replaces boundaries without recreating the map and resets edit history', async () => {
		const props = $state<ComponentProps<typeof AreaMap>>({
			boundary: { geometry: polygon },
			onchange: vi.fn()
		});
		await renderMap(props);
		mocks.draw.set({
			type: 'FeatureCollection',
			features: [{ type: 'Feature', properties: {}, geometry: otherPolygon }]
		});
		mocks.events.get('draw.update')?.();
		flushSync();
		expect(button('Undo')?.disabled).toBe(false);
		expect(props.onchange).toHaveBeenCalledOnce();

		props.boundary = { geometry: otherPolygon };
		flushSync();

		expect(mocks.Map).toHaveBeenCalledOnce();
		expect(mocks.Draw).toHaveBeenCalledOnce();
		expect(mocks.map.remove).not.toHaveBeenCalled();
		expect(mocks.draw.getAll().features[0].geometry).toEqual(otherPolygon);
		expect(button('Undo')?.disabled).toBe(true);
		expect(button('Redo')?.disabled).toBe(true);
		expect(mocks.map.fitBounds).toHaveBeenCalledTimes(2);
		expect(props.onchange).toHaveBeenCalledOnce();
	});

	it('does not replace geometry or reset history when edits are emitted', async () => {
		const props = $state<ComponentProps<typeof AreaMap>>({
			boundary: { geometry: polygon },
			onchange: vi.fn()
		});
		await renderMap(props);
		mocks.draw.set({
			type: 'FeatureCollection',
			features: [{ type: 'Feature', properties: {}, geometry: otherPolygon }]
		});
		mocks.events.get('draw.update')?.();
		flushSync();

		expect(mocks.draw.set).toHaveBeenCalledTimes(2);
		expect(mocks.map.fitBounds).toHaveBeenCalledOnce();
		expect(button('Undo')?.disabled).toBe(false);
		button('Undo')?.click();
		flushSync();
		expect(mocks.draw.getAll().features[0].geometry).toEqual(polygon);
		expect(button('Redo')?.disabled).toBe(false);
	});

	it('uses the latest boundary if selection changes before the map loads', async () => {
		const props = $state<ComponentProps<typeof AreaMap>>({
			boundary: { geometry: polygon },
			onchange: vi.fn()
		});
		await renderMap(props, false);
		props.boundary = { geometry: otherPolygon };
		flushSync();
		expect(mocks.draw.set).not.toHaveBeenCalled();

		mocks.events.get('load')?.();
		flushSync();
		expect(mocks.draw.getAll().features[0].geometry).toEqual(otherPolygon);
		expect(mocks.draw.set).toHaveBeenCalledOnce();
	});

	it('blocks editing while loading and clears polygons for an empty boundary', async () => {
		const props = $state<ComponentProps<typeof AreaMap>>({
			boundary: { geometry: polygon },
			onchange: vi.fn()
		});
		await renderMap(props);
		props.disabled = true;
		flushSync();
		expect(button('Draw polygon')?.disabled).toBe(true);
		expect(
			document.querySelector<HTMLDivElement>('[aria-label="Area boundary map"]')?.inert
		).toBe(true);
		mocks.events.get('draw.update')?.();
		expect(props.onchange).not.toHaveBeenCalled();

		props.boundary = { geometry: null };
		props.disabled = false;
		flushSync();
		expect(mocks.draw.getAll().features).toEqual([]);
		expect(button('Draw polygon')?.disabled).toBe(false);
		expect(mocks.Map).toHaveBeenCalledOnce();
	});
});
