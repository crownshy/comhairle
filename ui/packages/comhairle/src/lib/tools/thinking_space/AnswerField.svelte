<script lang="ts">
	import { m } from '$lib/paraglide/messages';
	import { Button } from '$lib/components/ui/button';
	import { Textarea } from '$lib/components/ui/textarea';
	import { cn } from '$lib/utils';
	import { countWords, MAX_ANSWER_WORDS } from './wordLimit';

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
	const wordCountId = `${fieldId}-word-count`;
	const overLimitId = `${fieldId}-over-limit`;

	let wordCount = $derived(countWords(value));
	let overLimit = $derived(wordCount > MAX_ANSWER_WORDS);
	let describedBy = $derived(overLimit ? `${wordCountId} ${overLimitId}` : wordCountId);

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
	aria-describedby={describedBy}
	aria-invalid={overLimit || undefined}
	{value}
	oninput={(e) => onInput(e.currentTarget.value)}
	placeholder={m.thinking_space_write_thoughts()}
	rows={4}
	class="text-base"
/>
{#if overLimit}
	<p id={overLimitId} role="alert" class="text-destructive text-base">
		{m.thinking_space_over_word_limit({ max: MAX_ANSWER_WORDS })}
	</p>
{/if}
<div class="flex items-center justify-between gap-3">
	<span id={wordCountId} class={cn('text-base', overLimit && 'text-destructive font-semibold')}>
		{m.thinking_space_word_count({ count: wordCount, max: MAX_ANSWER_WORDS })}
	</span>
	<Button onclick={handleContinue} disabled={!value.trim() || submitting}>
		{submitting ? `${m.saving()}…` : m.continue_()}
	</Button>
</div>
