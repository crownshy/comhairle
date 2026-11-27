import type { FeatureCollection, MultiPolygon, Polygon } from 'geojson';
import type { RegionAreaDto } from '@crownshy/api-client/api';
import { boundarySchema } from './import';

/** Build renderable boundaries without changing polygon rings or disconnected parts. */
export function areaFeatures(areas: RegionAreaDto[]): FeatureCollection<Polygon | MultiPolygon> {
	return {
		type: 'FeatureCollection',
		features: areas.flatMap((area) => {
			const geometry = boundarySchema.safeParse(area.areaGeometry);
			if (!geometry.success) return [];
			return [
				{
					type: 'Feature' as const,
					id: area.id,
					properties: {
						id: area.id,
						name: area.name || area.zipPrefix || 'Unnamed area'
					},
					geometry: geometry.data
				}
			];
		})
	};
}

type Bounds = { west: number; south: number; east: number; north: number };

/** Describe map bounds by centre and span, so a view across the antimeridian stays one request. */
export function viewportQuery({ west, south, east, north }: Bounds) {
	const lngSpan = Math.min(360, east - west);
	const latSpan = north - south;
	if (![west, south, east, north].every(Number.isFinite) || lngSpan <= 0 || latSpan <= 0) {
		return null;
	}
	return {
		viewport_center_lng: wrapLongitude((west + east) / 2),
		viewport_center_lat: (south + north) / 2,
		viewport_lng_span: lngSpan,
		viewport_lat_span: latSpan
	};
}

function wrapLongitude(longitude: number) {
	return ((((longitude + 180) % 360) + 360) % 360) - 180;
}

/** Toggle one association, preserving the order of the remaining selection. */
export function toggleArea(selectedIds: string[], id: string): string[] {
	const index = selectedIds.findIndex((selectedId) => selectedId === id);
	if (index < 0) {
		return selectedIds.concat(id);
	}
	return selectedIds.toSpliced(index, 1);
}
