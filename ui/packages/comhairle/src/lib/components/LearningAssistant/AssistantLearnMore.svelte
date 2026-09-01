<script lang="ts">
	/** The standing explanation of the assistant: where answers come from, that it can be
	 *  wrong, and what not to type. Folded away behind "Learn more" rather than sitting
	 *  above the input, where it was read past. */
	import * as m from '$lib/paraglide/messages';

	const entries = [
		{ q: m.learning_assistant_more_q_1(), a: m.learning_assistant_more_a_1() },
		{ q: m.learning_assistant_more_q_2(), a: m.learning_assistant_more_a_2() },
		{ q: m.learning_assistant_more_q_3(), a: m.learning_assistant_more_a_3() },
		{ q: m.learning_assistant_more_q_4(), a: m.learning_assistant_more_a_4() }
	];
</script>

<dl class="space-y-4">
	{#each entries as entry (entry.q)}
		<div>
			<dt class="text-foreground text-base font-semibold">{entry.q}</dt>
			<dd class="text-muted-foreground mt-1 text-base leading-relaxed">{entry.a}</dd>
		</div>
	{/each}
</dl>
