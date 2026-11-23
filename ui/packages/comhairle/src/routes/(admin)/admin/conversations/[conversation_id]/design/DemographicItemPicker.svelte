<script lang="ts">
	import { resolve } from '$app/paths';
	import { buttonVariants } from '$lib/components/ui/button';
	import SelectableOptionRow from '$lib/components/SelectableOptionRow.svelte';
	import type { PickerLoad, PublishedDemographicItem } from '$lib/demographicCatalogue';

	/**
	 * Lists the published questions or forms a demographic step can point at. Items are
	 * chosen here, so the step is only created once one is picked.
	 */
	type Props = {
		noun: 'question' | 'form';
		load: PickerLoad | undefined;
		selectedId: string | null;
		onSelect: (item: PublishedDemographicItem) => void;
	};
	let { noun, load, selectedId, onSelect }: Props = $props();
</script>

{#if load === undefined || load.status === 'loading'}
	<p class="text-muted-foreground text-base">Loading {noun}s…</p>
{:else if load.status === 'error'}
	<p class="text-destructive text-base">Could not load your {noun}s. Close and try again.</p>
{:else if load.items.length === 0}
	<div class="flex flex-col gap-3">
		<p class="text-muted-foreground text-base">
			Your hosting organisation has no published {noun}s yet. Publish one first, then come
			back to add it.
		</p>
		<a
			href={resolve('/(admin)/admin/demographic-forms')}
			class={buttonVariants({ variant: 'outline', size: 'sm' })}
		>
			Open demographic forms
		</a>
	</div>
{:else}
	<div class="flex flex-col gap-2.5">
		<h4 class="text-muted-foreground text-sm">CHOOSE A {noun.toUpperCase()}</h4>
		{#each load.items as item (item.id)}
			<SelectableOptionRow
				selected={selectedId === item.id}
				name={item.name}
				description={item.detail}
				onSelect={() => onSelect(item)}
			/>
		{/each}
	</div>
{/if}
