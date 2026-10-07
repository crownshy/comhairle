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

/** Toggle one association, preserving the order of the remaining selection. */
export function toggleArea(selectedIds: string[], id: string): string[] {
	const index = selectedIds.findIndex((selectedId) => selectedId === id);
	if (index < 0) {
		return selectedIds.slice().concat(id);
	}
	return selectedIds.toSpliced(index, 1);
}
