<script lang="ts">
	/** One answer's content: the answer itself, its inline citations, or the failure that
	 *  replaced it. Deliberately free of surrounding layout so the panel owns the frame. */
	import { AlertTriangle, RefreshCw } from 'lucide-svelte';
	import * as m from '$lib/paraglide/messages';
	import MessageWithReferences from '$lib/components/Chatbot/MessageWithReferences.svelte';
	import type { ReferenceChunk } from '$lib/api/chatClient.svelte';
	import type { QA } from './assistantState.svelte';

	let {
		qa,
		onOpenSource,
		onRetry,
		canRetry = false
	}: {
		qa: QA;
		onOpenSource: (chunk: ReferenceChunk) => void;
		onRetry?: () => void;
		canRetry?: boolean;
	} = $props();
</script>

{#if qa.error && !qa.answer}
	<div
		class="border-destructive/30 bg-destructive/5 flex items-start gap-3 rounded-xl border p-4"
	>
		<AlertTriangle class="text-destructive mt-0.5 h-5 w-5 shrink-0" />
		<div class="min-w-0 flex-1 text-base">
			<p class="text-destructive font-semibold">{m.learning_assistant_no_answer()}</p>
			<p class="text-foreground/80">{qa.error}</p>
			{#if canRetry && onRetry}
				<button
					type="button"
					class="border-destructive/40 text-destructive hover:bg-destructive/10 mt-3 inline-flex items-center gap-2 rounded-lg border px-3 py-1.5 text-base font-medium transition-colors"
					onclick={onRetry}
				>
					<RefreshCw class="h-4 w-4" />
					{m.try_again()}
				</button>
			{/if}
		</div>
	</div>
{:else if qa.answer}
	<div class="text-foreground text-base leading-relaxed">
		<MessageWithReferences content={qa.answer} reference={qa.reference} {onOpenSource} />
		{#if qa.streaming}
			<span class="bg-primary ml-0.5 inline-block h-4 w-1.5 animate-pulse align-middle"
			></span>
		{/if}
	</div>
{:else}
	<span class="text-muted-foreground inline-flex items-center gap-2 text-base">
		<span class="inline-flex items-center gap-1">
			<span class="bg-primary/60 h-2 w-2 animate-bounce rounded-full"></span>
			<span
				class="bg-primary/60 h-2 w-2 animate-bounce rounded-full"
				style="animation-delay: 0.15s"
			></span>
			<span
				class="bg-primary/60 h-2 w-2 animate-bounce rounded-full"
				style="animation-delay: 0.3s"
			></span>
		</span>
		{m.learning_assistant_thinking()}
	</span>
{/if}
