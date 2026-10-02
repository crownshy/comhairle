import type { LocalizedRegionDto, RegionType } from '@crownshy/api-client/api';
import { apiClient } from '@crownshy/api-client/client';
import { getLocale } from '$lib/paraglide/runtime';
import { saveTranslation } from '$lib/components/Translation/translationUtils';
import { tryCatchAsync } from '$lib/utils/errorHandling';
import { toggleArea } from './selection';

/** Own the region draft and persist text, metadata, and area associations. */
export function createRegionEditor() {
	let regions = $state<LocalizedRegionDto[]>([]);
	let editing = $state(false);
	let id = $state<string | null>(null);
	let name = $state('');
	let description = $state('');
	let regionType = $state<RegionType>('custom');
	let officialId = $state('');
	let areaIds = $state<string[]>([]);
	let loading = $state(false);
	let listing = $state(false);
	let saving = $state(false);
	let error = $state('');
	let saved = $state(false);
	let baseline = $state('');
	let requestId = 0;
	let disposed = false;
	const snapshot = $derived(
		JSON.stringify([name, description, regionType, officialId, [...areaIds].sort()])
	);
	const dirty = $derived(editing && snapshot !== baseline);

	async function loadRegions() {
		listing = true;
		const response = await tryCatchAsync(async () => {
			const records: LocalizedRegionDto[] = [];
			while (true) {
				const page = await apiClient.ListRegions({
					queries: { limit: 200, offset: records.length, created_at: 'asc' }
				});
				records.push(...page.records);
				if (!page.records.length || records.length >= page.total) return records;
			}
		});
		if (disposed) return;
		listing = false;
		if (response.err !== null) {
			error = 'Could not load regions. Try refreshing the list.';
			return;
		}
		regions = response.ok;
	}

	function reset() {
		requestId += 1;
		id = null;
		name = '';
		description = '';
		regionType = 'custom';
		officialId = '';
		areaIds = [];
		error = '';
		saved = false;
		loading = false;
		baseline = snapshot;
	}

	async function select(region: LocalizedRegionDto) {
		if (saving) return;
		reset();
		const currentRequest = requestId;
		id = region.id;
		editing = false;
		loading = true;
		const response = await tryCatchAsync(() =>
			Promise.all([
				apiClient.GetRegion({ params: { region_id: region.id } }),
				apiClient.GetRegionAreaLinks({ params: { region_id: region.id } })
			])
		);
		if (disposed || currentRequest !== requestId) return;
		loading = false;
		if (response.err !== null) {
			error = 'Could not load this region.';
			return;
		}
		const [current, links] = response.ok;
		name = current.name;
		description = current.description;
		regionType = current.region_type;
		officialId = current.official_id ?? '';
		areaIds = links.area_ids;
		baseline = snapshot;
		editing = true;
	}

	async function save() {
		if (!editing || saving || !name.trim()) return;
		saving = true;
		error = '';
		saved = false;
		const response = await tryCatchAsync(async () => {
			if (id === null) {
				const created = await apiClient.CreateRegion({
					name: name.trim(),
					description,
					region_type: regionType,
					official_id: officialId.trim() || null
				});
				id = created.id;
			} else {
				const updated = await apiClient.UpdateRegion(
					{ region_type: regionType, official_id: officialId.trim() },
					{ params: { region_id: id } }
				);
				await saveTranslation(updated.name, getLocale(), name.trim(), {
					requiresValidation: false
				});
				await saveTranslation(updated.description, getLocale(), description, {
					requiresValidation: false
				});
			}
			await apiClient.SetRegionAreaLinks(
				{ area_ids: areaIds },
				{ params: { region_id: id } }
			);
		});
		saving = false;
		if (disposed) return;
		if (response.err !== null) {
			error =
				'Could not save all changes. Some changes may have saved; your draft is retained for retry.';
			return;
		}
		name = name.trim();
		officialId = officialId.trim();
		baseline = snapshot;
		saved = true;
		await loadRegions();
	}

	async function remove() {
		if (!id || saving) return;
		saving = true;
		error = '';
		const regionId = id;
		const response = await tryCatchAsync(() =>
			apiClient.DeleteRegion(undefined, { params: { region_id: regionId } })
		);
		saving = false;
		if (disposed) return;
		if (response.err !== null) {
			error = 'Could not delete this region.';
			return;
		}
		reset();
		editing = false;
		await loadRegions();
	}

	return {
		get regions() {
			return regions;
		},
		get editing() {
			return editing;
		},
		get id() {
			return id;
		},
		get name() {
			return name;
		},
		set name(value: string) {
			name = value;
			saved = false;
		},
		get description() {
			return description;
		},
		set description(value: string) {
			description = value;
			saved = false;
		},
		get regionType() {
			return regionType;
		},
		set regionType(value: RegionType) {
			regionType = value;
			saved = false;
		},
		get officialId() {
			return officialId;
		},
		set officialId(value: string) {
			officialId = value;
			saved = false;
		},
		get areaIds() {
			return areaIds;
		},
		get loading() {
			return loading;
		},
		get listing() {
			return listing;
		},
		get saving() {
			return saving;
		},
		get error() {
			return error;
		},
		get saved() {
			return saved && !dirty;
		},
		get dirty() {
			return dirty;
		},
		toggle(id: string) {
			if (!saving) {
				areaIds = toggleArea(areaIds, id);
				saved = false;
			}
		},
		create() {
			if (!saving) {
				reset();
				editing = true;
			}
		},
		dispose() {
			disposed = true;
			requestId += 1;
		},
		loadRegions,
		select,
		save,
		remove
	};
}
