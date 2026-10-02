<script lang="ts">
	import * as m from '$lib/paraglide/messages';

	let { isSeed }: { isSeed: boolean } = $props();
</script>

<p
	class="mb-1 text-base font-semibold {isSeed
		? 'text-seed-highlight'
		: 'text-participant-highlight'}"
>
	{isSeed ? m.polis_seed_statement() : m.polis_participant_statement()}
</p>
