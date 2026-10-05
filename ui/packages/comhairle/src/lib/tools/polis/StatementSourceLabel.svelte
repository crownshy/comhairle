<script lang="ts">
	import { Megaphone, User } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';

	type Props = { isSeed: boolean };

	let { isSeed }: Props = $props();

	const Icon = $derived(isSeed ? Megaphone : User);
</script>

<p
	class="inline-flex items-start gap-2 rounded-2xl border px-2.5 py-1 text-base font-medium sm:items-center sm:rounded-full sm:px-3 {isSeed
		? 'border-seed-highlight text-seed-highlight'
		: 'border-participant-highlight text-participant-highlight'}"
>
	<Icon class="mt-0.5 size-5 shrink-0 sm:mt-0" aria-hidden="true" />
	{isSeed ? m.polis_seed_statement() : m.polis_participant_statement()}
</p>
