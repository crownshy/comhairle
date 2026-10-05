<script lang="ts">
	import { BadgeCheck, User } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	type Props = { isSeed: boolean };

	let { isSeed }: Props = $props();

	const Icon = $derived(isSeed ? BadgeCheck : User);
</script>

<p
	class="inline-flex items-center gap-2 rounded-full border px-3 py-1 text-base font-medium {isSeed
		? 'border-seed-highlight text-seed-highlight'
		: 'border-participant-highlight text-participant-highlight'}"
>
	<Icon class="size-5 shrink-0" aria-hidden="true" />
	{isSeed ? m.polis_seed_statement() : m.polis_participant_statement()}
</p>
