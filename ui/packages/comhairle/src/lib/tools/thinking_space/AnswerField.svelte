<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { countWords, MAX_ANSWER_WORDS, wordLimitDescribedBy, wordLimitIds } from './wordLimit';
	import WordCount from './WordCount.svelte';
	import OverWordLimitMessage from './OverWordLimitMessage.svelte';

	type Props = {
		value: string;
		/** Id of the element holding the question this field answers. */
		labelledBy: string;
		submitting: boolean;
		ref?: HTMLTextAreaElement | null;
		onInput: (value: string) => void;
		onSubmit: () => void;
	};

	let {
		value,
		labelledBy,
		submitting,
		ref = $bindable(null),
		onInput,
		onSubmit
	}: Props = $props();

	const fieldId = $props.id();
	const ids = wordLimitIds(fieldId);

	let wordCount = $derived(countWords(value));
	let overLimit = $derived(wordCount > MAX_ANSWER_WORDS);

	// Continue stays enabled over the limit so a press can send focus back to the field,
	// where the invalid state and the reason are read out.
	function handleContinue() {
		if (overLimit) {
			ref?.focus();
			return;
		}
		onSubmit();
	}
</script>

<Textarea
	bind:ref
	aria-labelledby={labelledBy}
	aria-describedby={wordLimitDescribedBy(fieldId, { countShown: true, overLimit })}
	aria-invalid={overLimit || undefined}
	{value}
	oninput={(e) => onInput(e.currentTarget.value)}
	placeholder={m.thinking_space_write_thoughts()}
	rows={4}
	class="text-base"
/>
{#if overLimit}
	<OverWordLimitMessage id={ids.overLimit} max={MAX_ANSWER_WORDS} />
{/if}
<div class="flex items-center justify-between gap-3">
	<WordCount id={ids.wordCount} count={wordCount} max={MAX_ANSWER_WORDS} />
	<Button onclick={handleContinue} disabled={!value.trim() || submitting}>
		{submitting ? `${m.saving()}…` : m.continue_()}
	</Button>
</div>
