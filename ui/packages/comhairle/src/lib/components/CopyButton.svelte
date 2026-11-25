<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { cn } from '$lib/utils';
	import { Check, Copy } from '@lucide/svelte';
	import type { Snippet } from 'svelte';
	import { copy } from 'svelte-copy';
	import { buttonVariants } from './ui/button';

	type Props = {
		copyText: string;
		children?: Snippet;
	};
	let { copyText, children }: Props = $props();

	const COPIED_FOR_MS = 2000;
	let copied = $state(false);
	let resetTimer: ReturnType<typeof setTimeout> | undefined;

	// INFO toasts are switched off app-wide, so the button itself has to show the copy worked.
	function showCopied() {
		copied = true;
		clearTimeout(resetTimer);
		resetTimer = setTimeout(() => (copied = false), COPIED_FOR_MS);
	}
</script>

<Tooltip.Provider>
	<Tooltip.Root>
		<Tooltip.Trigger>
			<button
				use:copy={{ text: copyText, onCopy: showCopied }}
				class={cn(
					buttonVariants.variants.variant.outline,
					'flex items-center gap-x-1 rounded-full p-2'
				)}
				aria-label={copied ? 'Copied' : 'Copy'}
			>
				{@render children?.()}
				{#if copied}
					<Check class="size-4 shrink-0" />
					<span class="text-xs font-semibold">Copied</span>
				{:else}
					<Copy class="size-4" />
				{/if}
			</button>
		</Tooltip.Trigger>
		<Tooltip.Content>{copied ? 'Copied' : 'Click to copy'}</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
