<script lang="ts">
	import type { ReconciliationCounts } from '@crownshy/api-client/api';
	type Props = { preview: ReconciliationCounts };
	let { preview }: Props = $props();
	const changes = $derived([
		{ label: 'Created', count: preview.created, ids: preview.created_ids },
		{ label: 'Updated', count: preview.updated, ids: preview.updated_ids },
		{ label: 'Retired', count: preview.retired, ids: preview.retired_ids },
		{ label: 'Restored', count: preview.restored, ids: preview.restored_ids },
		{ label: 'Repaired', count: preview.repaired.length, ids: preview.repaired },
		{
			label: 'Unmatched by tag rules',
			count: preview.unmatched_ids.length,
			ids: preview.unmatched_ids
		}
	]);
</script>

<section class="grid gap-4 rounded border p-4" aria-label="Reconciliation preview">
	<h3 class="text-xl font-semibold">Reconciliation preview</h3>
	<p class="text-muted-foreground">
		Nothing has been published yet. Review changes before activating. Existing linked Regions
		keep their identity and follow the imported boundaries.
	</p>
	<p>{preview.unchanged} unchanged</p>
	{#each changes as change (change.label)}
		<details>
			<summary class="cursor-pointer">{change.label}: {change.count}</summary>
			<ul class="mt-2 max-h-48 overflow-auto pl-6">
				{#each change.ids as id (id)}<li class="list-disc break-all">{id}</li>{/each}
			</ul>
		</details>
	{/each}
	{#if preview.retired}
		<p class="text-destructive">
			Publishing will retire {preview.retired} missing features. Review the retired identifiers
			above.
		</p>
	{/if}
	<details>
		<summary class="cursor-pointer">Managed tag changes: {preview.tag_changes.length}</summary>
		<ul class="mt-2 max-h-64 space-y-3 overflow-auto">
			{#each preview.tag_changes as change (change.external_id)}
				<li class="break-words">
					<strong>{change.external_id}</strong>: added {change.added.join(', ') ||
						'none'}; removed {change.removed.join(', ') || 'none'}
				</li>
			{/each}
		</ul>
	</details>
</section>
