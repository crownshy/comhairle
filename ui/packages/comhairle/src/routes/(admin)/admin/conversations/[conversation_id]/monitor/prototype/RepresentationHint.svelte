<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import ActionHint from './ActionHint.svelte';
	import { isUnderRepresented, type RepresentationDimension } from './sampleMonitorData';

	type Props = {
		dimensions: RepresentationDimension[];
		/** Opens the Demographic analysis card on the given demographic. */
		onView: (dimensionName: string) => void;
	};

	let { dimensions, onView }: Props = $props();

	/** Per demographic: groups you've heard from no one in, and groups below their share. */
	let gaps = $derived(
		dimensions
			.map((dimension) => ({
				name: dimension.name,
				missing: dimension.rows
					.filter((row) => row.participantPercent === 0 && row.populationPercent > 0)
					.map((row) => row.group),
				under: dimension.rows
					.filter((row) => row.participantPercent > 0 && isUnderRepresented(row))
					.map((row) => row.group)
			}))
			.filter((gap) => gap.missing.length + gap.under.length > 0)
	);

	let missingCount = $derived(gaps.reduce((sum, gap) => sum + gap.missing.length, 0));
	let underCount = $derived(gaps.reduce((sum, gap) => sum + gap.under.length, 0));

	let message = $derived.by(() => {
		const parts = [];
		if (missingCount)
			parts.push(`${missingCount} ${missingCount === 1 ? 'group is' : 'groups are'} missing`);
		if (underCount)
			parts.push(
				`${underCount} ${underCount === 1 ? 'group is' : 'groups are'} under-represented`
			);
		const names = gaps.map((gap) => gap.name.toLowerCase());
		const list =
			names.length > 1 ? `${names.slice(0, -1).join(', ')} and ${names.at(-1)}` : names[0];
		return `${parts.join(' and ')} across ${list}.`;
	});
</script>

<!-- Shown at the top of Recruitment while any group is missing or under-represented. -->
{#if gaps.length}
	<ActionHint>
		{message[0].toUpperCase() + message.slice(1)}
		{#snippet actions()}
			<Button
				class="rounded-full bg-orange-700 text-white hover:bg-orange-800"
				onclick={() => onView(gaps[0].name)}>View now</Button
			>
		{/snippet}
	</ActionHint>
{/if}
