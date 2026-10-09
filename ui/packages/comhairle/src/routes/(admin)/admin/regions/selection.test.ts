import { describe, expect, it } from 'vitest';
import type { RegionAreaDto } from '@crownshy/api-client/api';
import { areaFeatures, toggleArea } from './selection';

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
