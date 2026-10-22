<script lang="ts">
	import * as Tooltip from '$lib/components/ui/tooltip';
	import Icon from '@iconify/svelte';
	import { Check } from '@lucide/svelte';
	import type { Snippet } from 'svelte';
	import { copy } from 'svelte-copy';

	type Props = {
		copyText: string;
		children: Snippet;
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
		<Tooltip.Trigger class="w-full">
			<button
				use:copy={{ text: copyText, onCopy: showCopied }}
				class="flex w-full cursor-pointer flex-row items-center justify-center gap-x-2 outline-hidden"
				aria-label={copied ? 'Copied' : 'Copy'}
			>
				{@render children()}
				{#if copied}
					<Check class="text-primary size-4 shrink-0" />
					<span class="text-primary text-sm font-semibold">Copied</span>
				{:else}
					<Icon icon="solar:copy-bold" class="shrink-0" />
				{/if}
			</button>
		</Tooltip.Trigger>
		<Tooltip.Content>{copied ? 'Copied' : 'Click to copy'}</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
