import { z } from 'zod';
import type { MultiPolygon, Polygon } from 'geojson';
import type { CreateRegionArea } from '@crownshy/api-client/api';
import BinaryTree from '$lib/data-structures/BinaryTree';

const position = z.tuple([z.number().min(-180).max(180), z.number().min(-90).max(90)]);
const ring = z
	.array(position)
	.min(4)
	.refine(
		(points) => points[0][0] === points.at(-1)?.[0] && points[0][1] === points.at(-1)?.[1],
		'Boundary rings must be closed'
	);
const polygon = z.array(ring).min(1);

/** WGS84 boundary geometry; holes and disconnected polygons are retained. */
export const boundarySchema = z.discriminatedUnion('type', [
	z.object({ type: z.literal('Polygon'), coordinates: polygon }),
	z.object({ type: z.literal('MultiPolygon'), coordinates: z.array(polygon).min(1) })
]);
const featureSchema = z.object({
	type: z.literal('Feature'),
	id: z.union([z.string(), z.number()]).optional(),
	geometry: boundarySchema,
	properties: z.record(z.unknown()).nullable()
});
const collectionSchema = z.object({
	type: z.literal('FeatureCollection'),
	features: z.array(featureSchema).min(1),
	crs: z.unknown().optional()
});

/** Parsed boundary dataset awaiting metadata confirmation. */
export type BoundaryDataset = {
	features: z.infer<typeof featureSchema>[];
	fields: string[];
	nameField: string;
	tags: string[];
};

/** Normalize polygon geometry without flattening interior rings. */
export function multiPolygon(geometry: Polygon | MultiPolygon): MultiPolygon {
	return geometry.type === 'MultiPolygon'
		? geometry
		: { type: 'MultiPolygon', coordinates: [geometry.coordinates] };
}

/** Collect finite longitude/latitude pairs from possibly incomplete draw coordinates. */
export function positionsFromCoordinates(value: unknown): number[][] {
	if (!Array.isArray(value)) return [];
	const coordinates = value.slice(0, 2);
	if (
		coordinates.length === 2 &&
		coordinates.every(
			(coordinate) => typeof coordinate === 'number' && Number.isFinite(coordinate)
		)
	) {
		return [coordinates];
	}
	return value.flatMap(positionsFromCoordinates);
}

/** Detect common Census and ONS attribute conventions, preserving original string IDs. */
export function inspectBoundaries(input: unknown, filename: string): BoundaryDataset {
	const collection = collectionSchema.parse(input);
	if (collection.crs)
		throw new Error('GeoJSON must be WGS84 without a legacy CRS. Export as EPSG:4326.');
	const binaryTree = new BinaryTree<string, string>();
	for (let feature of collection.features) {
		for (let key of Object.keys(feature.properties ?? {})) {
			binaryTree.insert(key);
		}
	}
	const fields = binaryTree.toArray();
	const nameField =
		fields.find((field) => /^NAME(?:\d+)?$/i.test(field)) ??
		fields.find((field) => /^[A-Z]+\d{2}NM$/i.test(field)) ??
		fields.find((field) => /^(name|label|NAMELSAD|official_name)$/i.test(field)) ??
		'';
	const tags = [`dataset:${filename.replace(/\.(geojson|json|zip)$/i, '')}`];
	return { features: collection.features, fields, nameField, tags };
}

/** Convert an inspected dataset into import records with explicit field mappings. */
export function importRecords(
	dataset: BoundaryDataset,
	nameField: string,
	tags: string[]
): CreateRegionArea[] {
	return dataset.features.map((feature) => {
		const properties = feature.properties ?? {};
		const name = properties[nameField] as string;
		const propertyTags = Object.entries(properties)
			.filter(
				([field, value]) =>
					/^(STATEFP|COUNTYFP|STUSPS|LSAD|COUNTRY|TYPE)(\d+)?$/i.test(field) &&
					(typeof value === 'string' || typeof value === 'number')
			)
			.map(([field, value]) => `${field.toLowerCase()}:${value}`);
		return {
			name: typeof name === 'string' ? name : null,
			tags: Array.from(
				new Set(
					tags
						.concat(propertyTags)
						.map((tag) => tag.trim())
						.filter(Boolean)
				)
			),
			zip_prefix: null,
			area_geometry: multiPolygon(feature.geometry)
		};
	});
}

/** Parse GeoJSON or zipped Shapefiles, reprojecting Shapefiles through their PRJ definition. */
export async function readBoundaryFile(file: File): Promise<BoundaryDataset> {
	if (/\.zip$/i.test(file.name)) {
		const { default: shp } = await import('shpjs');
		const parsed = await shp(await file.arrayBuffer());
		if (Array.isArray(parsed))
			throw new Error('Upload a ZIP containing one boundary layer at a time.');
		return inspectBoundaries(parsed, file.name);
	}
	if (!/\.(geojson|json)$/i.test(file.name))
		throw new Error('Choose a GeoJSON file or zipped Shapefile.');
	return inspectBoundaries(JSON.parse(await file.text()), file.name);
}
