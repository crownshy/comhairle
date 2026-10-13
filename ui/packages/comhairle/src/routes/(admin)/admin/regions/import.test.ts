import { describe, expect, it } from 'vitest';
import { inspectBoundaries, importRecords, multiPolygon, positionsFromCoordinates } from './import';

const geometry = {
	type: 'MultiPolygon' as const,
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
const collection = (properties: Record<string, unknown>) => ({
	type: 'FeatureCollection',
	features: [{ type: 'Feature', properties, geometry }]
});

describe('boundary imports', () => {
	it('retains islands and holes and preserves Census IDs with leading zeroes', () => {
		const dataset = inspectBoundaries(
			collection({ GEOID: '00123', NAME: 'Example', STATEFP: '01' }),
			'counties.geojson'
		);
		const records = importRecords(dataset, dataset.nameField, dataset.tags);
		expect(records[0].name).toBe('Example');
		expect(records[0].area_geometry).toEqual(geometry);
		expect(records[0].tags).toContain('statefp:01');
	});
	it('detects ONS codes and names', () => {
		const dataset = inspectBoundaries(
			collection({ ONS_CODE: 'E09000001', NAME: 'City of London' }),
			'ons.json'
		);
		expect(dataset.nameField).toBe('NAME');
	});
	it('rejects non-boundaries and projected coordinates', () => {
		expect(() =>
			inspectBoundaries({ type: 'FeatureCollection', features: [] }, 'empty.json')
		).toThrow();
		const invalid = collection({ GEOID: '1' });
		invalid.features[0].geometry = {
			type: 'MultiPolygon',
			coordinates: [
				[
					[
						[1000, 0],
						[1001, 0],
						[1001, 1],
						[1000, 0]
					]
				]
			]
		};
		expect(() => inspectBoundaries(invalid, 'projected.json')).toThrow();
	});
	it('wraps Polygon without losing its holes', () => {
		expect(
			multiPolygon({ type: 'Polygon', coordinates: geometry.coordinates[0] }).coordinates
		).toEqual([geometry.coordinates[0]]);
	});
	it('collects valid bounds from incomplete draw coordinates', () => {
		expect(positionsFromCoordinates([[[1, 2], null, [3, 4]], undefined, [[null, 5]]])).toEqual([
			[1, 2],
			[3, 4]
		]);
	});
});
