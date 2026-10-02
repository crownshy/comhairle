<script lang="ts">
	import * as Popover from '$lib/components/ui/popover';
	import { Info } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	type Props = {
		isSeed: boolean;
	};

	let { isSeed }: Props = $props();

	const label = $derived(isSeed ? m.polis_seed_statement() : m.polis_participant_statement());
	const explanation = $derived(
		isSeed ? m.polis_seed_statement_explanation() : m.polis_participant_statement_explanation()
	);
</script>

<!-- A popover rather than a tooltip so the explanation opens on tap as well as click. -->
<Popover.Root>
	<Popover.Trigger
		class="ml-auto flex items-center gap-1.5 rounded text-base font-medium underline decoration-dotted underline-offset-4 {isSeed
			? 'text-seed-highlight'
			: 'text-muted-foreground'}"
		aria-label={m.polis_statement_label_info({ label })}
	>
		{label}
		<Info class="h-4 w-4" aria-hidden="true" />
	</Popover.Trigger>
	<Popover.Content align="end" class="max-w-xs text-base">
		{explanation}
	</Popover.Content>
</Popover.Root>
