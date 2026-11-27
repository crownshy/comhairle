import { describe, expect, it } from 'vitest';
import type { RegionAreaDto } from '@crownshy/api-client/api';
import { areaFeatures, toggleArea, viewportQuery } from './selection';

describe('viewport query', () => {
	it('caps the longitude span when the view is wider than the world', () => {
		expect(viewportQuery({ west: -297, south: -72, east: 62, north: 84 })).toEqual({
			viewport_center_lng: -117.5,
			viewport_center_lat: 6,
			viewport_lng_span: 359,
			viewport_lat_span: 156
		});
		expect(viewportQuery({ west: -400, south: 0, east: 400, north: 10 })).toMatchObject({
			viewport_lng_span: 360
		});
	});
	it('keeps a view across the antimeridian as one centred span', () => {
		expect(viewportQuery({ west: 175, south: -20, east: 185, north: -10 })).toEqual({
			viewport_center_lng: -180,
			viewport_center_lat: -15,
			viewport_lng_span: 10,
			viewport_lat_span: 10
		});
		expect(viewportQuery({ west: -370, south: 50, east: -360, north: 60 })).toMatchObject({
			viewport_center_lng: -5
		});
	});
});

describe('region area selection', () => {
	it('toggles only the requested area, including overlapping selections', () => {
		expect(toggleArea(['outer'], 'overlap')).toEqual(['outer', 'overlap']);
		expect(toggleArea(['outer', 'overlap'], 'outer')).toEqual(['overlap']);
	});
	it('retains holes and exclaves and omits missing or invalid boundaries', () => {
		const geometry = {
			type: 'MultiPolygon',
			coordinates: [
				[
					[
						[0, 0],
						[10, 0],
						[10, 10],
						[0, 0]
					],
					[
						[2, 1],
						[3, 1],
						[3, 2],
						[2, 1]
					]
				],
				[
					[
						[20, 0],
						[21, 0],
						[21, 1],
						[20, 0]
					]
				]
			]
		};
		const area: RegionAreaDto = {
			id: 'outer',
			name: 'Outer',
			tags: [],
			createdAt: '',
			areaGeometry: geometry
		};
		const collection = areaFeatures([
			area,
			{ ...area, id: 'empty', areaGeometry: null },
			{ ...area, id: 'invalid', areaGeometry: { type: 'Point' } }
		]);
		expect(collection.features).toHaveLength(1);
		expect(collection.features[0]).toMatchObject({ id: 'outer', geometry });
	});
});
